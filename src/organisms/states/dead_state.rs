use std::time::Duration;

use crate::environment_awareness::EnvironmentAwareness;

use super::{
    organism_state::{OrganismState, StateRunResult},
    shared_state::SharedState,
};

pub struct DeadState {}

impl DeadState {
    pub fn new() -> Self {
        Self {}
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }
}

impl OrganismState for DeadState {
    fn initialize(_shared_state: &mut SharedState) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn run(
        &mut self,
        _shared_state: &mut SharedState,
        _deltaa: Duration,
        _environment_awareness: &EnvironmentAwareness,
    ) -> StateRunResult {
        StateRunResult::none_same()
    }

    fn name(&self, _shared_state: &SharedState) -> String {
        "dead".into()
    }
}
