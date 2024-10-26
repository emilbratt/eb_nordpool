use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum State {
    Preliminary, // This variant might be wrong, must find out later..
    Final,
}

impl State {
    pub fn is_preliminary(&self) -> bool {
        matches!(self, State::Preliminary)
    }
}
