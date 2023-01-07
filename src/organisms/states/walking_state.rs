use std::time::Duration;

use ggez::mint::Point2;
use rand::Rng;

use super::{
    idle_state::IdleState,
    organism_state::{OrganismState, StateRunResult},
    shared_state::SharedState,
};

const NEW_TARGET_DISTANCE: [f32; 2] = [1.0, 5.0];

#[derive(Clone, Copy)]
pub struct WalkingState {
    target: Point2<f32>,
}

impl WalkingState {
    fn get_distance(&self, shared_state: &SharedState) -> f32 {
        let to_target = vecmath::vec2_sub(self.target.into(), shared_state.position.into());
        vecmath::vec2_len(to_target)
    }
}

impl OrganismState for WalkingState {
    fn initialize(shared_state: &mut SharedState) -> Self {
        Self {
            target: pick_random_target(shared_state.position),
        }
    }

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateRunResult {
        let new_pos = calculate_position(
            delta,
            shared_state.position,
            self.target,
            shared_state.species.walk_speed_s,
        );
        shared_state.position = new_pos;

        if new_pos.eq(&self.target) {
            return StateRunResult::none_next(Box::new(IdleState::initialize(shared_state)));
        }

        StateRunResult::none_same()
    }

    fn name(&self, shared_state: &SharedState) -> String {
        format!("walking ({:.2}m)", self.get_distance(shared_state))
    }
}

fn pick_random_target(current_pos: Point2<f32>) -> Point2<f32> {
    let distance: f32 =
        rand::thread_rng().gen_range(NEW_TARGET_DISTANCE[0]..=NEW_TARGET_DISTANCE[1]);
    let angle = rand::thread_rng().gen_range(0f32..std::f32::consts::TAU); // 0 to 360 but in radians

    let direction_vector = create_direction_vector(angle);
    let target_relative = vecmath::vec2_scale(direction_vector, distance);
    let new_target = vecmath::vec2_add(target_relative, current_pos.into());

    new_target.into()
}

fn create_direction_vector(angle: f32) -> [f32; 2] {
    let forward_vector = vecmath::vec2_normalized([0f32, 1f32]);

    [
        forward_vector[0] * angle.cos() - forward_vector[1] * angle.sin(),
        forward_vector[0] * angle.sin() + forward_vector[1] * angle.cos(),
    ]
}

fn calculate_position(
    delta: Duration,
    current_pos: Point2<f32>,
    target: Point2<f32>,
    walking_speed_s: f32,
) -> Point2<f32> {
    let to_target = vecmath::vec2_sub(target.into(), current_pos.into());
    let distance = vecmath::vec2_len(to_target);

    if distance <= walking_speed_s * delta.as_secs_f32() {
        target
    } else {
        let direction_to_target = vecmath::vec2_normalized(to_target);
        let direction_to_target_per_time =
            vecmath::vec2_scale(direction_to_target, walking_speed_s * delta.as_secs_f32());
        vecmath::vec2_add(current_pos.into(), direction_to_target_per_time).into()
    }
}
