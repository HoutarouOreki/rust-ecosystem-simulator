use std::time::Duration;

use super::{
    organism_state::{OrganismState, StateTransition},
    shared_state::SharedState,
};

pub struct EatingState {}

impl OrganismState for EatingState {
    fn initialize(_shared_state: &mut SharedState) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn run(&mut self, _shared_state: &mut SharedState, _delta: Duration) -> StateTransition {
        StateTransition::Same
    }
}
