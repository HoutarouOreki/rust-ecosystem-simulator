use std::time::Duration;

use rand::Rng;

use crate::organisms::{
    organism_result::OrganismResult,
    states::{idle_state::IdleState, organism_state::StateTransition},
};

use super::{
    organism_state::{AwarenessOfOtherOrganism, OrganismState, StateRunResult},
    shared_state::SharedState,
};

const REPRODUCING_DURATION_S: f32 = 6.0;

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
        _awareness_of_others: &[AwarenessOfOtherOrganism],
    ) -> super::organism_state::StateRunResult {
        if self.time_left < delta {
            shared_state.on_had_children();
            return StateRunResult {
                organism_result: OrganismResult::HadChildren {
                    amount: rand::thread_rng().gen_range(1..=3),
                },
                state_transition: StateTransition::Next(Box::new(IdleState::initialize(
                    shared_state,
                ))),
            };
        }

        self.time_left -= delta;
        StateRunResult::none_same()
    }

    fn name(&self, _shared_state: &SharedState) -> String {
        format!(
            "reproducing ({:.0}%)",
            100.0 * (REPRODUCING_DURATION_S - self.time_left.as_secs_f32())
                / REPRODUCING_DURATION_S
        )
    }
}
