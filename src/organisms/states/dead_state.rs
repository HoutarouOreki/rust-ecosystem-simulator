use std::time::Duration;

use super::{
    organism_state::{ForeignerInfo, OrganismState, StateRunResult},
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
        _foreigners_info: &[ForeignerInfo],
    ) -> StateRunResult {
        StateRunResult::none_same()
    }

    fn name(&self, _shared_state: &SharedState) -> String {
        "dead".into()
    }
}
