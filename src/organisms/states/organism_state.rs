use std::time::Duration;

use crate::organisms::organism_result::OrganismResult;

use super::shared_state::SharedState;

pub trait OrganismState {
    fn initialize(shared_state: &mut SharedState) -> Self
    where
        Self: Sized;

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateRunResult;

    fn init_boxed(shared_state: &mut SharedState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self::initialize(shared_state))
    }
}

pub enum StateTransition {
    Same,
    Next(Box<dyn OrganismState>),
}

pub struct StateRunResult {
    pub organism_result: OrganismResult,
    pub state_transition: StateTransition,
}

impl StateRunResult {
    pub fn new(organism_result: OrganismResult, state_transition: StateTransition) -> Self {
        Self {
            organism_result,
            state_transition,
        }
    }

    pub fn none_same() -> Self {
        Self {
            organism_result: OrganismResult::None,
            state_transition: StateTransition::Same,
        }
    }

    pub fn none_next(next_state: Box<dyn OrganismState>) -> Self {
        Self {
            organism_result: OrganismResult::None,
            state_transition: StateTransition::Next(next_state),
        }
    }
}
