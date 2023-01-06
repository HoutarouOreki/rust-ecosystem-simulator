use std::time::Duration;

use crate::organisms::{
    organism_result::OrganismResult,
    states::{idle_state::IdleState, organism_state::StateTransition},
};

use super::organism_state::{OrganismState, StateRunResult};

const REPRODUCING_DURATION_S: f32 = 4.0;

pub struct ReproducingState {
    time_left: Duration,
}

impl OrganismState for ReproducingState {
    fn initialize(_shared_state: &mut super::shared_state::SharedState) -> Self
    where
        Self: Sized,
    {
        Self {
            time_left: Duration::from_secs_f32(REPRODUCING_DURATION_S),
        }
    }

    fn run(
        &mut self,
        shared_state: &mut super::shared_state::SharedState,
        delta: std::time::Duration,
    ) -> super::organism_state::StateRunResult {
        if self.time_left < delta {
            return StateRunResult {
                organism_result: OrganismResult::HadChildren { amount: 1 },
                state_transition: StateTransition::Next(Box::new(IdleState::initialize(
                    shared_state,
                ))),
            };
            // somehow add a child to environment
        }

        self.time_left -= delta;
        StateRunResult::none_same()
    }
}
