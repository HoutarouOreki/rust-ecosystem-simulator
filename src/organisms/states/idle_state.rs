use std::time::Duration;

use rand::Rng;

// these are u32 'cause Rng::gen_ratio supports u32
const EAT_CHANCE: u32 = 4;
const WALK_CHANCE: u32 = 2;
const REPRODUCE_CHANCE: u32 = 7;

use super::{
    eating_state::EatingState,
    organism_state::{OrganismState, StateTransition},
    reproducing_state::ReproducingState,
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

    fn total_chance(shared_state: &SharedState) -> u32 {
        let mut sum = 0;

        if shared_state.can_walk() {
            sum += WALK_CHANCE;
        }
        if shared_state.can_reproduce() {
            sum += REPRODUCE_CHANCE;
        }
        if shared_state.can_eat() {
            sum += EAT_CHANCE;
        }

        sum
    }

    fn pick_new_state(
        shared_state: &SharedState,
    ) -> fn(&mut SharedState) -> Box<dyn OrganismState> {
        let mut total_chance = Self::total_chance(shared_state);
        let mut rng = rand::thread_rng();

        if shared_state.can_walk() && rng.gen_ratio(WALK_CHANCE, total_chance) {
            return |st| Box::new(WalkingState::initialize(st));
        } else {
            total_chance -= WALK_CHANCE;
        }

        if shared_state.can_eat() && rng.gen_ratio(EAT_CHANCE, total_chance) {
            return |st| Box::new(EatingState::initialize(st));
        } else {
            total_chance -= EAT_CHANCE;
        }

        if shared_state.can_reproduce() && rng.gen_ratio(REPRODUCE_CHANCE, total_chance) {
            return |st| Box::new(ReproducingState::initialize(st));
        }

        |st| Box::new(IdleState::initialize(st))
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
