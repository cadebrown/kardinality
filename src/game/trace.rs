use serde::{Deserialize, Serialize};

use crate::vm::Effect;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct StateDelta {
    pub score: i64,
    pub bankroll: i64,
    pub acc: i64,
    pub len_source: i64,
    pub len_deck: i64,
    pub len_hand: i64,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum TraceEvent {
    Action {
        action: String,
    },
    CardStart {
        index: usize,
        name: String,
        script: String,
        budget: usize,
        cost: usize,
    },
    CardEnd {
        index: usize,
        name: String,
        delta: StateDelta,
    },
    Call {
        name: String,
        args: Vec<String>,
    },
    EffectApplied {
        effect: Effect,
    },
    Info(String),
    Error(String),
}
