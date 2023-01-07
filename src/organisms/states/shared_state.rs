use core::f32;
use std::time::Duration;

use ggez::mint::Point2;

use crate::organisms::species::Species;

pub struct SharedState {
    pub position: Point2<f32>,
    age: Duration,
    energy: f32,
    pub health: f32,
    pub species: Species,
}

impl SharedState {
    pub fn new_default(species: Species) -> Self {
        Self {
            position: Point2 { x: 0.0, y: 0.0 },
            energy: species.max_energy,
            health: species.max_health,
            species,
            age: Duration::ZERO,
        }
    }

    pub fn new(
        position: Point2<f32>,
        energy: f32,
        health: f32,
        species: Species,
        age: Duration,
    ) -> Self {
        Self {
            position,
            energy,
            health,
            species,
            age,
        }
    }

    pub fn can_walk(&self) -> bool {
        self.species.walk_speed_s > 0.0
    }

    pub fn can_reproduce(&self) -> bool {
        self.energy >= self.species.cost_of_birth
    }

    pub fn can_eat(&self) -> bool {
        self.energy < self.species.max_energy
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn increase_energy(&mut self, amount: f32) {
        self.energy = f32::min(self.species.max_energy, self.energy + amount);
    }

    pub fn increase_age(&mut self) {
        self.age += Duration::from_secs(1);
    }

    pub fn age(&self) -> Duration {
        self.age
    }
}
