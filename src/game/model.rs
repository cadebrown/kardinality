use serde::{Deserialize, Serialize};

use crate::vm::Limits;

use super::cards::{self, CardDef};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Phase {
    InLevel,
    Reward,
    GameOver,
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


