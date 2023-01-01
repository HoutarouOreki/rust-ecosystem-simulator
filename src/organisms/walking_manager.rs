use std::time::Duration;

use ggez::mint::Point2;

use rand::distributions::Uniform;
use rand::Rng;

pub struct WalkingManager {
    target: Option<Point2<f32>>,
    idle_time_left: Option<Duration>,
    rng: rand::rngs::ThreadRng,
}

const WALKING_SPEED_PER_SECOND: f32 = 0.8;

const IDLE_TIME_S: [f32; 2] = [3.0, 7.0];

const NEW_TARGET_DISTANCE: [f32; 2] = [1.0, 5.0];

impl WalkingManager {
    pub fn new() -> Self {
        Self {
            target: Option::None,
            idle_time_left: Option::None,
            rng: rand::thread_rng(),
        }
    }

    pub fn simulate_and_calculate_new_pos(
        &mut self,
        current_pos: Point2<f32>,
        delta: Duration,
    ) -> Point2<f32> {
        if let Some(target) = self.target {
            // println!("\nThere is a target: {}, {}", target.x, target.y);
            let new_pos = calculate_position(delta, current_pos, target);
            // println!("Old pos: {}, {}", current_pos.x, current_pos.y);
            // println!("New pos: {}, {}", new_pos.x, new_pos.y);
            if new_pos.eq(&target) {
                self.target = Option::None;
                self.idle_time_left = Option::Some(Duration::from_secs_f32(
                    self.rng
                        .sample(Uniform::new(IDLE_TIME_S[0], IDLE_TIME_S[1])),
                ));
            }
            new_pos
        } else if let Some(idle_time_left) = self.idle_time_left {
            // println!("Idle time left: {}", idle_time_left);
            self.handle_idle_time(delta, idle_time_left, current_pos);
            current_pos
        } else {
            self.idle_time_left = Option::Some(Duration::from_secs_f32(
                self.rng
                    .sample(Uniform::new(IDLE_TIME_S[0], IDLE_TIME_S[1])),
            ));
            current_pos
        }
    }

    fn handle_idle_time(
        &mut self,
        delta: Duration,
        idle_time_left: Duration,
        current_pos: Point2<f32>,
    ) {
        if idle_time_left <= delta {
            self.idle_time_left = Option::None;
            self.pick_new_target(current_pos);
        } else {
            self.idle_time_left = Option::Some(idle_time_left - delta);
        }
    }

    fn pick_new_target(&mut self, current_pos: Point2<f32>) {
        let distance: f32 = self
            .rng
            .sample(Uniform::new(NEW_TARGET_DISTANCE[0], NEW_TARGET_DISTANCE[1]));
        let angle = self.rng.gen_range(0f32..std::f32::consts::TAU); // 0 to 360 but in radians

        let direction_vector = create_direction_vector(angle);
        let target_relative = vecmath::vec2_scale(direction_vector, distance);
        let new_target = vecmath::vec2_add(target_relative, current_pos.into());

        self.target = Option::Some(new_target.into());
    }
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
) -> Point2<f32> {
    let to_target = vecmath::vec2_sub(target.into(), current_pos.into());
    // println!("To target: {}, {}", to_target[0], to_target[1]);
    let distance = vecmath::vec2_len(to_target);
    // println!("Distance: {}", distance);

    // println!(
    //     "Walking speed * delta_s: {}",
    //     WALKING_SPEED_PER_SECOND * delta_s
    // );
    if distance <= WALKING_SPEED_PER_SECOND * delta.as_secs_f32() {
        // println!(">target: {}, {}", target.x, target.y);
        target
    } else {
        let direction_to_target = vecmath::vec2_normalized(to_target);
        // println!(
        //     "Direction to target: {}, {}",
        //     direction_to_target[0], direction_to_target[1]
        // );
        let direction_to_target_per_time =
            vecmath::vec2_scale(direction_to_target, WALKING_SPEED_PER_SECOND * delta.as_secs_f32());
        vecmath::vec2_add(current_pos.into(), direction_to_target_per_time).into()
    }
}
