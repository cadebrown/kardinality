use rand::seq::SliceRandom;
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha8Rng;
use thiserror::Error;

use std::collections::VecDeque;

use crate::kardlang::{effective_len, parse_program};
use crate::vm::{Effect, Limits, Machine, VmContext, VmError};

use crate::game::{CardInstance, GameState, Phase, RunMode, TraceEvent, cards, puzzles};

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    NewRun { seed: u64 },
    StartPuzzle { id: String },
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

    #[error("unknown puzzle id: {0}")]
    UnknownPuzzle(String),

    #[error("card script cost {cost} exceeds budget {budget}: {name}")]
    CardOverBudget {
        name: String,
        cost: usize,
        budget: usize,
    },

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
        // Draw pile ("source") is generated from weighted level-aware rules.
        let mut draw_pile: Vec<CardInstance> = Vec::new();
        let mut next_id: u64 = 1;
        for def_id in cards::generate_source_ids(seed, 1) {
            draw_pile.push(CardInstance::new(next_id, def_id));
            next_id += 1;
        }

        let mut engine = Self::with_deck(seed, draw_pile, Limits::default());
        engine.state.target_score = target_for_level(engine.state.level);

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
        let next_id = deck
            .iter()
            .map(|c| c.id)
            .max()
            .unwrap_or(0)
            .saturating_add(1);
        Self {
            state: GameState::new(deck, limits),
            rng,
            next_id,
        }
    }

    pub fn dispatch(&mut self, action: Action) -> Result<(), GameError> {
        self.state.trace.push(TraceEvent::Action {
            action: format!("{action:?}"),
        });

        match action {
            Action::NewRun { seed } => {
                *self = Self::new(seed);
                Ok(())
            }
            Action::StartPuzzle { id } => self.start_puzzle(&id),
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

    fn start_puzzle(&mut self, id: &str) -> Result<(), GameError> {
        let puzzle = puzzles::get(id).ok_or_else(|| GameError::UnknownPuzzle(id.to_string()))?;

        for card_id in puzzle
            .source_ids
            .iter()
            .chain(puzzle.collection_ids.iter())
            .chain(puzzle.hand_ids.iter())
        {
            if cards::get(card_id).is_none() {
                return Err(GameError::UnknownCardDef((*card_id).to_string()));
            }
        }

        let seed = stable_seed_from_id(puzzle.id);
        let source = puzzle
            .source_ids
            .iter()
            .enumerate()
            .map(|(i, id)| CardInstance::new((i + 1) as u64, *id))
            .collect::<Vec<_>>();

        let mut next = Self::with_deck(seed, source, self.state.limits);
        next.state.mode = RunMode::Puzzle;
        next.state.level = puzzle.start_level.max(1);
        next.state.bankroll = puzzle.start_bankroll;
        next.state.score = puzzle.start_score;
        next.state.acc = 0;
        next.state.target_score = puzzle.target_score.max(1);
        next.state.phase = Phase::InLevel;
        next.state.turn = 0;
        next.state.puzzle_id = Some(puzzle.id.to_string());
        next.state.puzzle_title = Some(puzzle.name.to_string());
        next.state.puzzle_blurb = Some(puzzle.blurb.to_string());
        next.state.puzzle_hint = Some(puzzle.hint.to_string());
        next.state.puzzle_theme = Some(puzzle.theme.to_string());
        next.state.puzzle_play_limit = puzzle.play_limit;
        next.state.puzzle_bankroll_goal = puzzle.goal_bankroll;
        next.state.puzzle_solved = false;
        next.state.puzzle_failed = false;
        next.state.puzzle_message = Some(format!("{}: {}", puzzle.name, puzzle.blurb));

        next.state.collection = puzzle
            .collection_ids
            .iter()
            .map(|id| next.new_card(*id))
            .collect();
        next.state.hand = puzzle
            .hand_ids
            .iter()
            .map(|id| next.new_card(*id))
            .collect();

        next.state
            .trace
            .push(TraceEvent::Info(format!("Puzzle loaded: {}", puzzle.name)));
        next.state
            .trace
            .push(TraceEvent::Info(format!("Hint: {}", puzzle.hint)));
        next.state.trace.push(TraceEvent::Info(format!(
            "Goal: score >= {}{}",
            next.state.target_score,
            next.state
                .puzzle_bankroll_goal
                .map(|g| format!(", bankroll >= {g}"))
                .unwrap_or_default()
        )));
        if let Some(limit) = next.state.puzzle_play_limit {
            next.state
                .trace
                .push(TraceEvent::Info(format!("Play limit: {limit}")));
        }

        *self = next;
        Ok(())
    }

    fn play_hand(&mut self) -> Result<(), GameError> {
        if self.state.phase != Phase::InLevel {
            return Ok(());
        }

        self.state.turn = self.state.turn.saturating_add(1);

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
                    self.state.trace.push(TraceEvent::EffectApplied { effect });
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

        if self.state.mode == RunMode::Puzzle {
            self.update_puzzle_outcome();
        } else if self.state.score >= self.state.target_score {
            self.advance_classic_level();
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
                    self.state.trace.push(TraceEvent::Info(
                        "clone/again: no last played card".to_string(),
                    ));
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
                    .or_else(|| {
                        self.state
                            .collection
                            .iter_mut()
                            .find(|c| c.id == last.card_id)
                    });
                let Some(target) = target else {
                    self.state.trace.push(TraceEvent::Info(
                        "mutate: last played card not in pile/deck".to_string(),
                    ));
                    return;
                };

                if let Some(new_id) = cards::roll_card_id_for_level(&mut self.rng, self.state.level)
                {
                    let old = target.def_id.clone();
                    target.def_id = new_id.to_string();
                    self.state.trace.push(TraceEvent::Info(format!(
                        "mutate: {} → {}",
                        old, target.def_id
                    )));
                }
            }
        }
    }

    fn advance_classic_level(&mut self) {
        let previous_level = self.state.level;
        self.state.level = self.state.level.saturating_add(1);
        self.state.target_score = target_for_level(self.state.level);

        // Add a small "booster" of newly generated cards every clear.
        let mut generated = 0usize;
        let booster = cards::generate_source_ids_with_count(
            self.rng.random::<u64>(),
            self.state.level,
            booster_count_for_level(self.state.level),
        );
        for id in booster {
            let card = self.new_card(id);
            self.state.deck.push(card);
            generated += 1;
        }
        self.state.deck.shuffle(&mut self.rng);

        self.state.trace.push(TraceEvent::Info(format!(
            "Level {previous_level} cleared! Next target: {} (+{generated} source cards)",
            self.state.target_score
        )));
    }

    fn update_puzzle_outcome(&mut self) {
        let score_ok = self.state.score >= self.state.target_score;
        let bankroll_ok = self
            .state
            .puzzle_bankroll_goal
            .is_none_or(|goal| self.state.bankroll >= goal);

        if score_ok && bankroll_ok {
            if !self.state.puzzle_solved {
                self.state.puzzle_solved = true;
                self.state.phase = Phase::Reward;
                let msg = format!(
                    "Puzzle solved in {} play(s). Great sequencing.",
                    self.state.turn
                );
                self.state.puzzle_message = Some(msg.clone());
                self.state.trace.push(TraceEvent::Info(msg));
            }
            return;
        }

        if let Some(limit) = self.state.puzzle_play_limit
            && self.state.turn >= limit
            && !self.state.puzzle_failed
        {
            self.state.puzzle_failed = true;
            self.state.phase = Phase::GameOver;
            let hint = self
                .state
                .puzzle_hint
                .as_deref()
                .unwrap_or("Try a different sequence.");
            let msg = format!("Puzzle failed: out of plays ({limit}). Hint: {hint}");
            self.state.puzzle_message = Some(msg.clone());
            self.state.trace.push(TraceEvent::Info(msg));
            return;
        }

        let mut status = format!(
            "Puzzle progress: score {}/{}",
            self.state.score, self.state.target_score
        );
        if let Some(goal) = self.state.puzzle_bankroll_goal {
            status.push_str(&format!(", bankroll {}/{}", self.state.bankroll, goal));
        }
        if let Some(limit) = self.state.puzzle_play_limit {
            let left = limit.saturating_sub(self.state.turn);
            status.push_str(&format!(", plays left {left}"));
        }
        self.state.puzzle_message = Some(status);
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

fn target_for_level(level: u32) -> i64 {
    let level = level.max(1) as i64;
    let base = 10i64;
    let linear = level.saturating_sub(1).saturating_mul(12);
    let curve = level
        .saturating_sub(1)
        .saturating_mul(level.saturating_sub(2))
        .saturating_mul(2);
    base.saturating_add(linear).saturating_add(curve)
}

fn booster_count_for_level(level: u32) -> usize {
    (3 + (level.max(1) as usize / 2)).min(8)
}

fn stable_seed_from_id(id: &str) -> u64 {
    // Stable FNV-1a hash so puzzle seeds are reproducible across targets.
    let mut h: u64 = 0xcbf2_9ce4_8422_2325;
    for b in id.as_bytes() {
        h ^= *b as u64;
        h = h.wrapping_mul(0x1000_0000_01b3);
    }
    h
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
            "len_deck" | "len_pool" | "len_collection" | "D" => {
                Some(self.state.collection.len() as i64)
            }
            "len_source" | "len_draw" | "S" => Some(self.state.deck.len() as i64),
            "len_hand" | "H" => Some(self.state.hand.len() as i64),
            "len_pile" | "len_discard" | "P" => Some(self.state.pile.len() as i64),
            "deck" => Some(self.state.collection.len() as i64),
            "hand" => Some(self.state.hand.len() as i64),
            "lvl" | "level" | "L" => Some(self.state.level as i64),
            "acc" | "A" => Some(self.state.acc),
            "bankroll" | "money" | "B" => Some(self.state.bankroll),
            "score" | "Q" => Some(self.state.score),
            "target" | "T" => Some(self.state.target_score),
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
        let deck = vec![CardInstance::new(1, "tap_score")];
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

    #[test]
    fn jam_two_adds_score_and_draws_once() {
        let deck = vec![CardInstance::new(1, "tap_bank")];
        let mut engine = Engine::with_deck(1, deck, Limits::default());
        engine.state.hand = vec![CardInstance::new(99, "jam_two")];

        engine.dispatch(Action::PlayHand).unwrap();

        assert_eq!(engine.state.score, 2);
        assert_eq!(engine.state.collection.len(), 1);
        assert_eq!(engine.state.pile.len(), 1);
    }

    #[test]
    fn clone_one_replays_the_previous_card() {
        let mut engine = Engine::with_deck(2, Vec::new(), Limits::default());
        engine.state.hand = vec![
            CardInstance::new(1, "tap_score"),
            CardInstance::new(2, "clone_one"),
        ];

        engine.dispatch(Action::PlayHand).unwrap();

        let replayed_score_cards = engine
            .state
            .pile
            .iter()
            .filter(|c| c.def_id == "tap_score")
            .count();
        assert_eq!(engine.state.score, 4);
        assert_eq!(replayed_score_cards, 2);
    }

    #[test]
    fn cash_two_converts_bankroll_into_score() {
        let mut engine = Engine::with_deck(3, Vec::new(), Limits::default());
        engine.state.hand = vec![CardInstance::new(1, "cash_two")];

        engine.dispatch(Action::PlayHand).unwrap();

        assert_eq!(engine.state.score, 2);
        assert_eq!(engine.state.bankroll, 8);
    }

    #[test]
    fn clearing_a_level_scales_target_and_generates_more_source_cards() {
        let mut engine = Engine::with_deck(4, Vec::new(), Limits::default());
        engine.state.score = engine.state.target_score;
        engine.state.hand = vec![CardInstance::new(1, "tap_bank")];
        engine.state.mode = RunMode::Classic;

        engine.dispatch(Action::PlayHand).unwrap();

        assert_eq!(engine.state.level, 2);
        assert_eq!(engine.state.target_score, target_for_level(2));
        assert!(
            engine.state.deck.len() >= booster_count_for_level(2),
            "expected level clear to generate booster cards"
        );
    }

    #[test]
    fn start_puzzle_sets_mode_hint_and_goal_state() {
        let mut engine = Engine::new(0);
        engine
            .dispatch(Action::StartPuzzle {
                id: "lesson_score_ping".to_string(),
            })
            .unwrap();

        assert_eq!(engine.state.mode, RunMode::Puzzle);
        assert_eq!(engine.state.target_score, 2);
        assert_eq!(engine.state.turn, 0);
        assert_eq!(engine.state.phase, Phase::InLevel);
        assert_eq!(engine.state.puzzle_id.as_deref(), Some("lesson_score_ping"));
        assert_eq!(engine.state.hand.len(), 1);
        assert!(engine.state.puzzle_hint.is_some());
    }

    #[test]
    fn puzzle_can_be_solved_in_one_play() {
        let mut engine = Engine::new(0);
        engine
            .dispatch(Action::StartPuzzle {
                id: "lesson_meta_clone".to_string(),
            })
            .unwrap();

        engine.dispatch(Action::PlayHand).unwrap();

        assert!(engine.state.puzzle_solved);
        assert!(!engine.state.puzzle_failed);
        assert_eq!(engine.state.phase, Phase::Reward);
        assert_eq!(engine.state.score, 6);
    }

    #[test]
    fn puzzle_fail_state_triggers_when_limit_is_spent() {
        let mut engine = Engine::new(0);
        engine
            .dispatch(Action::StartPuzzle {
                id: "lesson_score_ping".to_string(),
            })
            .unwrap();

        // Empty hand means no scoring this turn, so a 1-turn puzzle should fail.
        engine.state.hand.clear();
        engine.dispatch(Action::PlayHand).unwrap();

        assert!(engine.state.puzzle_failed);
        assert!(!engine.state.puzzle_solved);
        assert_eq!(engine.state.phase, Phase::GameOver);
        assert!(
            engine
                .state
                .puzzle_message
                .as_deref()
                .is_some_and(|m| m.contains("out of plays"))
        );
    }

    #[test]
    fn unknown_puzzle_id_returns_error() {
        let mut engine = Engine::new(0);
        let err = engine
            .dispatch(Action::StartPuzzle {
                id: "nope".to_string(),
            })
            .unwrap_err();
        assert!(matches!(err, GameError::UnknownPuzzle(_)));
    }

    #[test]
    fn puzzle_money_loop_hits_score_and_bankroll_goals() {
        let mut engine = Engine::new(0);
        engine
            .dispatch(Action::StartPuzzle {
                id: "lesson_money_loop".to_string(),
            })
            .unwrap();

        engine.dispatch(Action::PlayHand).unwrap();

        assert!(engine.state.puzzle_solved);
        assert_eq!(engine.state.score, 4);
        assert!(engine.state.bankroll >= 12);
    }

    #[test]
    fn puzzle_draw_math_reaches_expected_score() {
        let mut engine = Engine::new(0);
        engine
            .dispatch(Action::StartPuzzle {
                id: "lesson_draw_math".to_string(),
            })
            .unwrap();

        engine.dispatch(Action::PlayHand).unwrap();

        assert!(engine.state.puzzle_solved);
        assert_eq!(engine.state.score, 10);
    }
}
