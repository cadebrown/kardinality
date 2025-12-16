use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Limits {
    pub max_steps: usize,
    pub max_loop_iters: usize,
}

impl Default for Limits {
    fn default() -> Self {
        Self {
            max_steps: 200,
            max_loop_iters: 50,
        }
    }
}


