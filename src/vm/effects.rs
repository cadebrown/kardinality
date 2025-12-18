use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum Effect {
    AddScore(i64),
    AddBankroll(i64),
    MulBankroll(i64),
    Draw(i64),
    SetAcc(i64),
    Clone(i64),
    Again(i64),
    Mutate,
}
