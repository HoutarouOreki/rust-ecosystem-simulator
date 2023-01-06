use std::time::Duration;

use ggez::graphics::Color;

#[derive(Clone)]
pub struct Species {
    pub name: String,
    pub max_energy: f32,
    pub max_health: f32,
    pub max_age: Duration,
    pub cost_of_birth: f32,
    pub walk_speed_s: f32,
    pub can_eat_organisms: bool,
    pub photosynthesis_rate_s: f32,
    pub color: Color,
}
