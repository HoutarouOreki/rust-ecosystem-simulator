use std::time::Duration;

use rand::Rng;

// these are u32 'cause Rng::gen_ratio supports u32
const HUNT_CHANCE: u32 = 16;
const WALK_CHANCE: u32 = 10;
const REPRODUCE_CHANCE: u32 = 54;

use crate::environment_awareness::EnvironmentAwareness;

use super::{
    hunting_state::HuntingState,
    organism_state::{OrganismState, StateRunResult},
    reproducing_state::ReproducingState,
    shared_state::SharedState,
    walking_state::WalkingState,
};

#[derive(Clone, Copy)]
pub struct IdleState {
    duration: Duration,
    target_duration: Duration,
}

const IDLE_TIME_S: [f32; 2] = [1.0, 3.0];

impl IdleState {
    pub fn new() -> Self {
        Self {
            duration: Duration::ZERO,
            target_duration: Duration::from_secs_f32(
                rand::thread_rng().gen_range(IDLE_TIME_S[0]..=IDLE_TIME_S[1]),
            ),
        }
    }

    pub fn new_boxed() -> Box<Self> {
        Box::new(Self::new())
    }

    fn total_chance(shared_state: &SharedState) -> u32 {
        let mut sum = 0;

        if shared_state.can_walk() {
            sum += WALK_CHANCE;
        }
        if shared_state.can_reproduce() {
            sum += REPRODUCE_CHANCE;
        }
        if shared_state.can_hunt() {
            sum += HUNT_CHANCE;
        }

        sum
    }

    fn pick_new_state(
        shared_state: &SharedState,
    ) -> fn(&mut SharedState) -> Box<dyn OrganismState> {
        let total_chance = &mut Self::total_chance(shared_state);

        if shared_state.can_walk() && ratio(WALK_CHANCE, total_chance) {
            return |st| WalkingState::init_boxed(st);
        }

        if shared_state.can_hunt() && ratio(HUNT_CHANCE, total_chance) {
            return |st| HuntingState::init_boxed(st);
        }

        if shared_state.can_reproduce() && ratio(REPRODUCE_CHANCE, total_chance) {
            return |st| ReproducingState::init_boxed(st);
        }

        |st| IdleState::init_boxed(st)
    }
}

pub fn ratio(numerator: u32, denominator: &mut u32) -> bool {
    if *denominator == 0 {
        false
    } else if rand::thread_rng().gen_ratio(numerator, *denominator) {
        true
    } else {
        *denominator -= numerator;
        false
    }
}

impl OrganismState for IdleState {
    fn initialize(_shared_state: &mut SharedState) -> Self {
        Self::new()
    }

    fn run(
        &mut self,
        shared_state: &mut SharedState,
        delta: Duration,
        _environment_awareness: &EnvironmentAwareness,
    ) -> StateRunResult {
        self.duration += delta;
        if self.duration >= self.target_duration {
            StateRunResult::none_next(Self::pick_new_state(shared_state)(shared_state))
        } else {
            StateRunResult::none_same()
        }
    }

    fn name(&self, _shared_state: &SharedState) -> String {
        format!(
            "idling ({:.0}%)",
            self.duration.as_secs_f32() / self.target_duration.as_secs_f32() * 100.0
        )
    }
}
