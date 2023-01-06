use std::time::Duration;

use rand::Rng;

use super::{
    organism_state::{OrganismState, StateTransition},
    shared_state::SharedState,
    walking_state::WalkingState,
};

#[derive(Clone, Copy)]
pub struct IdleState {
    duration: Duration,
    target_duration: Duration,
}

const IDLE_TIME_S: [f32; 2] = [3.0, 7.0];

impl IdleState {
    pub fn new() -> Self {
        Self {
            duration: Duration::ZERO,
            target_duration: Duration::from_secs_f32(
                rand::thread_rng().gen_range(IDLE_TIME_S[0]..=IDLE_TIME_S[1]),
            ),
        }
    }
}

impl OrganismState for IdleState {
    fn initialize(_shared_state: &mut SharedState) -> Self {
        Self::new()
    }

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateTransition {
        self.duration += delta;
        if self.duration >= self.target_duration {
            StateTransition::Next(Box::new(WalkingState::initialize(shared_state)))
        } else {
            StateTransition::Same
        }
    }
}
