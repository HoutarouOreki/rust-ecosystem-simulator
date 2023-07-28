use std::time::Duration;

use ggez::graphics::Color;
use serde::{Deserialize, Serialize};

#[derive(Clone, Serialize, Deserialize)]
pub struct Species {
    pub name: String,
    pub max_energy: f32,
    pub max_health: f32,
    pub max_age: Duration,
    pub energy_cost_of_birth: f32,
    pub health_cost_of_birth: f32,
    pub walk_speed_s: f32,
    pub photosynthesis_rate_s: f32,
    pub color: Color,
    pub eats: Nutrition,
    pub contained_nutrition: Nutrition,
    pub eyesight_distance: f32,
    pub birth_distance: f32,
    pub birth_immunity: Duration,
    pub eating_distance: f32,
    pub max_per_meter: f32,
    pub hunting_behavior: HuntingBehavior,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum Nutrition {
    None,
    Plant,
    Meat,
    Corpse,
}

#[derive(Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum HuntingBehavior {
    Closest,
    Random,
}
