use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Final,
    Preliminary,
}

impl State {
    pub fn is_final(&self) -> bool {
        matches!(self, Self::Final)
    }

    pub fn is_preliminary(&self) -> bool {
        matches!(self, Self::Preliminary)
    }
}
