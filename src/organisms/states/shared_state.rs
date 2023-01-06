use ggez::mint::Point2;

use crate::organisms::species::{Species};

pub struct SharedState {
    pub position: Point2<f32>,
    pub energy: u32,
    pub health: u32,
    pub species: Species,
}

impl SharedState {
    pub fn can_walk(&self) -> bool {
        self.species.walk_speed_s > 0.0
    }

    pub fn can_reproduce(&self) -> bool {
        self.energy >= self.species.cost_of_birth
    }

    pub fn can_eat(&self) -> bool {
        self.energy < self.species.max_energy
    }
}
