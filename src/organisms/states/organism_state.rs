use std::time::Duration;

use super::shared_state::SharedState;

pub trait OrganismState {
    fn initialize(shared_state: &mut SharedState) -> Self
    where
        Self: Sized;

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateTransition;
}

pub enum StateTransition {
    Same,
    Next(Box<dyn OrganismState>),
}
