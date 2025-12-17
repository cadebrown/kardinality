use rand::prelude::IndexedRandom;
use rand::seq::SliceRandom;
use rand::SeedableRng;
use rand_chacha::ChaCha8Rng;
use thiserror::Error;

use std::collections::VecDeque;

use crate::kardlang::{effective_len, parse_program};
use crate::vm::{Effect, Limits, Machine, VmContext, VmError};

use crate::game::{cards, CardInstance, GameState, Phase, TraceEvent};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    NewRun { seed: u64 },
    DrawToCollection { count: usize },
    MoveCollectionToHand { index: usize },
    MoveHandToCollection { index: usize },
    ReorderCollection { from: usize, to: usize },
    ReorderHand { from: usize, to: usize },
    SwapCollection { a: usize, b: usize },
    SwapHand { a: usize, b: usize },
    PlayHand,
    ClearTrace,
}

#[derive(Debug, Error)]
pub enum GameError {
    #[error("kardlang parse error: {0}")]
    Parse(#[from] crate::kardlang::ParseError),

    #[error("vm error: {0}")]
    Vm(#[from] VmError),

    #[error("unknown card definition id: {0}")]
    UnknownCardDef(String),

    #[error("card script cost {cost} exceeds budget {budget}: {name}")]
    CardOverBudget { name: String, cost: usize, budget: usize },

    #[error("cannot draw a hand: deck and pile are empty")]
    NoCards,
}

#[derive(Debug)]
pub struct Engine {
    pub state: GameState,
    rng: ChaCha8Rng,
    next_id: u64,
}

impl Engine {
    pub fn new(seed: u64) -> Self {
        // Draw pile ("source") is a shuffled pool of all known cards (with repeats).
        let mut draw_pile: Vec<CardInstance> = Vec::new();
        let mut next_id: u64 = 1;
        for _ in 0..4 {
            for def in cards::catalog() {
                draw_pile.push(CardInstance::new(next_id, def.id));
                next_id += 1;
            }
        }

        let mut engine = Self::with_deck(seed, draw_pile, Limits::default());

        // Player starts with a small starter deck.
        engine.state.collection = cards::starter_deck_ids()
            .iter()
            .map(|id| engine.new_card(*id))
            .collect();

        engine
    }

    pub fn with_deck(seed: u64, mut deck: Vec<CardInstance>, limits: Limits) -> Self {
        let mut rng = ChaCha8Rng::seed_from_u64(seed);
        deck.shuffle(&mut rng);
        let next_id = deck.iter().map(|c| c.id).max().unwrap_or(0).saturating_add(1);
        Self {
            state: GameState::new(deck, limits),
            rng,
            next_id,
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), GameError> {
        self.state
            .trace
            .push(TraceEvent::Action { action: format!("{action:?}") });

        match action {
            Action::NewRun { seed } => {
                *self = Self::new(seed);
                Ok(())
            }
            Action::DrawToCollection { count } => self.draw_to_collection(count),
            Action::MoveCollectionToHand { index } => {
                if let Some(card) = take_at(&mut self.state.collection, index) {
                    self.state.hand.push(card);
                }
                Ok(())
            }
            Action::MoveHandToCollection { index } => {
                if let Some(card) = take_at(&mut self.state.hand, index) {
                    self.state.collection.push(card);
                }
                Ok(())
            }
            Action::ReorderCollection { from, to } => {
                move_within(&mut self.state.collection, from, to);
                Ok(())
            }
            Action::ReorderHand { from, to } => {
                move_within(&mut self.state.hand, from, to);
                Ok(())
            }
            Action::SwapCollection { a, b } => {
                swap_within(&mut self.state.collection, a, b);
                Ok(())
            }
            Action::SwapHand { a, b } => {
                swap_within(&mut self.state.hand, a, b);
                Ok(())
            }
            Action::PlayHand => self.play_hand(),
            Action::ClearTrace => {
                self.state.trace.clear();
                Ok(())
            }
        }
    }

    fn new_card(&mut self, def_id: &str) -> CardInstance {
        let id = self.next_id;
        self.next_id = self.next_id.saturating_add(1);
        CardInstance::new(id, def_id)
    }

    fn draw_to_collection(&mut self, count: usize) -> Result<(), GameError> {
        for _ in 0..count {
            if let Some(card) = self.draw_one(true)? {
                self.state.collection.push(card);
            }
        }
        Ok(())
    }

    fn draw_to_collection_source_only(&mut self, count: usize) -> Result<(), GameError> {
        for _ in 0..count {
            if let Some(card) = self.draw_one(false)? {
                self.state.collection.push(card);
            }
        }
        Ok(())
    }

    fn draw_one(&mut self, allow_recycle: bool) -> Result<Option<CardInstance>, GameError> {
        if self.state.deck.is_empty() {
            if !allow_recycle {
                return Ok(None);
            }
            if self.state.pile.is_empty() {
                return Err(GameError::NoCards);
            }

            self.state.deck.append(&mut self.state.pile);
            self.state.deck.shuffle(&mut self.rng);
        }

        Ok(self.state.deck.pop())
    }

    fn play_hand(&mut self) -> Result<(), GameError> {
        if self.state.phase != Phase::InLevel {
            return Ok(());
        }

        let mut vm = Machine::new(self.state.limits);

        let mut queue: VecDeque<CardInstance> = std::mem::take(&mut self.state.hand).into();
        let mut exec_index: usize = 0;

        while let Some(card) = queue.pop_front() {
            let def = card
                .def()
                .ok_or_else(|| GameError::UnknownCardDef(card.def_id.clone()))?;

            let before = Snapshot::capture(&self.state);

            let cost = effective_len(def.script);
            self.state.trace.push(TraceEvent::CardStart {
                index: exec_index,
                name: def.name.to_string(),
                script: def.script.to_string(),
                budget: def.budget,
                cost,
            });

            if cost > def.budget {
                return Err(GameError::CardOverBudget {
                    name: def.name.to_string(),
                    cost,
                    budget: def.budget,
                });
            }

            let program = parse_program(def.script)?;
            let mut post_queue: Vec<CardInstance> = Vec::new();

            for call in &program.calls {
                let args = call.args.iter().map(expr_to_string).collect::<Vec<_>>();

                self.state.trace.push(TraceEvent::Call {
                    name: call.name.clone(),
                    args,
                });

                let ctx = GameCtx { state: &self.state };
                let effects = vm.eval_call(call, &ctx)?;
                for effect in effects {
                    self.apply_effect_for_hand(&effect, &mut post_queue);
                    self.state
                        .trace
                        .push(TraceEvent::EffectApplied { effect });
                }
            }

            // After execution, cards go to the pile (discard).
            self.state.pile.push(card.clone());

            // Track full history for cards like clone/again/mutate.
            self.state.history.push(crate::game::HistoryEntry {
                card_id: card.id,
                def_id: card.def_id.clone(),
            });

            let after = Snapshot::capture(&self.state);
            self.state.trace.push(TraceEvent::CardEnd {
                index: exec_index,
                name: def.name.to_string(),
                delta: after.delta_from(before),
            });

            // Queue any extra executions to run immediately after this card.
            for c in post_queue.into_iter().rev() {
                queue.push_front(c);
            }

            exec_index = exec_index.saturating_add(1);
        }

        // Always draw 1 card after playing a hand (soft reward / pacing).
        {
            let effect = Effect::Draw(1);
            // Source-only (do not consume the pile).
            let _ = self.draw_to_collection_source_only(1);
            self.state.trace.push(TraceEvent::EffectApplied { effect });
        }

        if self.state.score >= self.state.target_score {
            self.state.level += 1;
            self.state.target_score += 10;
            self.state.trace.push(TraceEvent::Info(format!(
                "Level cleared! Next target: {}",
                self.state.target_score
            )));
        }

        Ok(())
    }

    fn apply_effect_for_hand(&mut self, effect: &Effect, post_queue: &mut Vec<CardInstance>) {
        match effect {
            Effect::AddScore(n) => self.state.score += *n,
            Effect::AddBankroll(n) => self.state.bankroll += *n,
            Effect::MulBankroll(n) => self.state.bankroll *= *n,
            Effect::Draw(n) => {
                let count: usize = (*n).clamp(0, 25) as usize;
                if let Err(e) = self.draw_to_collection(count) {
                    self.state.trace.push(TraceEvent::Error(e.to_string()));
                }
            }
            Effect::SetAcc(v) => self.state.acc = *v,
            Effect::Clone(n) | Effect::Again(n) => {
                let count: usize = (*n).clamp(0, 12) as usize;
                if count == 0 {
                    return;
                }

                let Some(last) = self.state.history.last() else {
                    self.state
                        .trace
                        .push(TraceEvent::Info("clone/again: no last played card".to_string()));
                    return;
                };

                // If the last played card was mutated, clone its current def_id from the pile/deck.
                let def_id = self
                    .state
                    .pile
                    .iter()
                    .find(|c| c.id == last.card_id)
                    .or_else(|| self.state.collection.iter().find(|c| c.id == last.card_id))
                    .map(|c| c.def_id.clone())
                    .unwrap_or_else(|| last.def_id.clone());

                for _ in 0..count {
                    post_queue.push(self.new_card(&def_id));
                }
            }
            Effect::Mutate => {
                let Some(last) = self.state.history.last() else {
                    self.state
                        .trace
                        .push(TraceEvent::Info("mutate: no last played card".to_string()));
                    return;
                };

                let target = self
                    .state
                    .pile
                    .iter_mut()
                    .find(|c| c.id == last.card_id)
                    .or_else(|| self.state.collection.iter_mut().find(|c| c.id == last.card_id));
                let Some(target) = target else {
                    self.state.trace.push(TraceEvent::Info(
                        "mutate: last played card not in pile/deck".to_string(),
                    ));
                    return;
                };

                if let Some(new_def) = cards::catalog().choose(&mut self.rng) {
                    let old = target.def_id.clone();
                    target.def_id = new_def.id.to_string();
                    self.state.trace.push(TraceEvent::Info(format!(
                        "mutate: {} → {}",
                        old, target.def_id
                    )));
                }
            }
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Snapshot {
    score: i64,
    bankroll: i64,
    acc: i64,
    len_source: i64,
    len_deck: i64,
    len_hand: i64,
}

impl Snapshot {
    fn capture(state: &GameState) -> Self {
        Self {
            score: state.score,
            bankroll: state.bankroll,
            acc: state.acc,
            len_source: state.deck.len() as i64,
            len_deck: state.collection.len() as i64,
            len_hand: state.hand.len() as i64,
        }
    }

    fn delta_from(self, before: Snapshot) -> crate::game::trace::StateDelta {
        crate::game::trace::StateDelta {
            score: self.score - before.score,
            bankroll: self.bankroll - before.bankroll,
            acc: self.acc - before.acc,
            len_source: self.len_source - before.len_source,
            len_deck: self.len_deck - before.len_deck,
            len_hand: self.len_hand - before.len_hand,
        }
    }
}

fn take_at<T>(v: &mut Vec<T>, index: usize) -> Option<T> {
    if index >= v.len() {
        return None;
    }
    Some(v.remove(index))
}

fn move_within<T>(v: &mut Vec<T>, from: usize, to: usize) {
    if from >= v.len() || from == to {
        return;
    }
    let item = v.remove(from);
    // Allow "insert at end" by clamping to the new len (after removal).
    let to = to.min(v.len());
    v.insert(to, item);
}

fn swap_within<T>(v: &mut Vec<T>, a: usize, b: usize) {
    if a >= v.len() || b >= v.len() || a == b {
        return;
    }
    v.swap(a, b);
}

struct GameCtx<'a> {
    state: &'a GameState,
}

impl VmContext for GameCtx<'_> {
    fn get(&self, name: &str) -> Option<i64> {
        match name {
            // Terminology:
            // - "deck" is the player's owned deck (selection pool)
            // - "source" is the generator/draw pile we pull new cards from
            "len_deck" | "len_pool" | "len_collection" => Some(self.state.collection.len() as i64),
            "len_source" | "len_draw" => Some(self.state.deck.len() as i64),
            "len_hand" => Some(self.state.hand.len() as i64),
            "len_pile" | "len_discard" => Some(self.state.pile.len() as i64),
            "deck" => Some(self.state.collection.len() as i64),
            "hand" => Some(self.state.hand.len() as i64),
            "lvl" => Some(self.state.level as i64),
            "acc" => Some(self.state.acc),
            "bankroll" => Some(self.state.bankroll),
            "level" => Some(self.state.level as i64),
            "target" => Some(self.state.target_score),
            "max_step" | "max_steps" => Some(self.state.limits.max_steps as i64),
            "max_loop" | "max_loop_iters" => Some(self.state.limits.max_loop_iters as i64),
            _ => None,
        }
    }
}

fn expr_to_string(expr: &crate::kardlang::Expr) -> String {
    use crate::kardlang::{BinOp, Expr};
    match expr {
        Expr::Int(n, _) => n.to_string(),
        Expr::Var(name, _) => name.clone(),
        Expr::Group(inner, _) => format!("({})", expr_to_string(inner)),
        Expr::Binary { op, lhs, rhs, .. } => {
            let op_str = match op {
                BinOp::Add => "+",
                BinOp::Mul => "*",
            };
            format!("{}{}{}", expr_to_string(lhs), op_str, expr_to_string(rhs))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn draw_then_play_applies_effects_and_moves_cards_to_pile() {
        let deck = vec![CardInstance::new(1, "score_2")];
        let mut engine = Engine::with_deck(123, deck, Limits::default());

        engine
            .dispatch(Action::DrawToCollection { count: 1 })
            .unwrap();
        assert_eq!(engine.state.collection.len(), 1);
        assert_eq!(engine.state.pile.len(), 0);

        engine
            .dispatch(Action::MoveCollectionToHand { index: 0 })
            .unwrap();
        assert_eq!(engine.state.hand.len(), 1);

        engine.dispatch(Action::PlayHand).unwrap();
        assert_eq!(engine.state.score, 2);
        assert_eq!(engine.state.hand.len(), 0);
        assert_eq!(engine.state.collection.len(), 0);
        assert_eq!(engine.state.pile.len(), 1);
    }

    #[test]
    fn engine_new_starts_with_starter_deck() {
        let engine = Engine::new(0);
        let expected = cards::starter_deck_ids().len();
        assert_eq!(engine.state.collection.len(), expected);
        assert_eq!(engine.state.hand.len(), 0);

        for c in &engine.state.collection {
            assert!(
                c.def().is_some(),
                "starter card def missing for id={}",
                c.def_id
            );
        }
    }
}


