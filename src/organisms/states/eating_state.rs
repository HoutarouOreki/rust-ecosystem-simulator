use std::time::Duration;

use super::{
    idle_state::IdleState,
    organism_state::{OrganismState, StateRunResult},
    shared_state::SharedState,
};

pub struct EatingState {
    time_remaining: Duration,
}

const EATING_DURATION_S: f32 = 2.5;
const ENERGY_FROM_EATING: f32 = 20.0;

impl OrganismState for EatingState {
    fn initialize(_shared_state: &mut SharedState) -> Self
    where
        Self: Sized,
    {
        Self {
            time_remaining: Duration::from_secs_f32(EATING_DURATION_S),
        }
    }

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateRunResult {
        if delta > self.time_remaining {
            shared_state.increase_energy(ENERGY_FROM_EATING);
            return StateRunResult::none_next(IdleState::init_boxed(shared_state));
        }

        self.time_remaining -= delta;
        StateRunResult::none_same()
    }

    fn name(&self, _shared_state: &SharedState) -> String {
        format!(
            "eating ({:.0}%)",
            100.0 * (EATING_DURATION_S - self.time_remaining.as_secs_f32()) / EATING_DURATION_S
        )
    }
}
