use serde::{Deserialize, Serialize};

use crate::vm::Limits;

use super::cards::{self, CardDef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    InLevel,
    Reward,
    GameOver,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum RunMode {
    Classic,
    Puzzle,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HistoryEntry {
    pub card_id: u64,
    pub def_id: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct CardInstance {
    pub id: u64,
    pub def_id: String,
}

impl CardInstance {
    pub fn new(id: u64, def_id: impl Into<String>) -> Self {
        Self {
            id,
            def_id: def_id.into(),
        }
    }

    pub fn def(&self) -> Option<&'static CardDef> {
        cards::get(&self.def_id)
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct GameState {
    pub bankroll: i64,
    pub score: i64,
    pub acc: i64,
    pub level: u32,
    pub target_score: i64,
    pub phase: Phase,
    pub mode: RunMode,
    pub turn: u32,
    pub puzzle_id: Option<String>,
    pub puzzle_title: Option<String>,
    pub puzzle_blurb: Option<String>,
    pub puzzle_hint: Option<String>,
    pub puzzle_theme: Option<String>,
    pub puzzle_play_limit: Option<u32>,
    pub puzzle_bankroll_goal: Option<i64>,
    pub puzzle_solved: bool,
    pub puzzle_failed: bool,
    pub puzzle_message: Option<String>,

    /// Draw pile. Hidden-ish, but still deterministic and reorderable for debug later.
    pub deck: Vec<CardInstance>,

    /// Cards currently available to be selected into the active hand ("program").
    pub collection: Vec<CardInstance>,

    /// Active hand/program to execute in order.
    pub hand: Vec<CardInstance>,

    /// Executed card pile (discard). Cards played from Hand are placed here.
    pub pile: Vec<CardInstance>,

    /// Full execution history (oldest first). Used for cards like `clone()`/`again()`/`mutate()`.
    pub history: Vec<HistoryEntry>,

    pub trace: Vec<crate::game::TraceEvent>,

    pub limits: Limits,
}

impl GameState {
    pub fn new(deck: Vec<CardInstance>, limits: Limits) -> Self {
        Self {
            bankroll: 10,
            score: 0,
            acc: 0,
            level: 1,
            target_score: 10,
            phase: Phase::InLevel,
            mode: RunMode::Classic,
            turn: 0,
            puzzle_id: None,
            puzzle_title: None,
            puzzle_blurb: None,
            puzzle_hint: None,
            puzzle_theme: None,
            puzzle_play_limit: None,
            puzzle_bankroll_goal: None,
            puzzle_solved: false,
            puzzle_failed: false,
            puzzle_message: None,
            deck,
            collection: Vec::new(),
            hand: Vec::new(),
            pile: Vec::new(),
            history: Vec::new(),
            trace: Vec::new(),
            limits,
        }
    }
}
