use std::time::Duration;

use rand::Rng;

use crate::organisms::organism::Organism;

use super::{organism_state::OrganismState, walking_state::WalkingState};

#[derive(Clone, Copy)]
pub struct IdleState {
    duration: Duration,
    target_duration: Duration,
}

impl IdleState {
    pub fn new() -> Self {
        Self {
            duration: Duration::ZERO,
            target_duration: Duration::ZERO,
        }
    }
}

const IDLE_TIME_S: [f32; 2] = [3.0, 7.0];

impl OrganismState for IdleState {
    fn initialize(_organism: &mut crate::organisms::organism::Organism) -> Self {
        Self {
            duration: Duration::ZERO,
            target_duration: Duration::from_secs_f32(
                rand::thread_rng().gen_range(IDLE_TIME_S[0]..=IDLE_TIME_S[1]),
            ),
        }
    }

    fn run(&mut self, organism: &mut Organism, delta: Duration) -> Box<dyn OrganismState> {
        self.duration += delta;
        if self.duration >= self.target_duration {
            Box::new(WalkingState::initialize(organism))
        } else {
            Box::new(*self)
        }
    }
}
