use core::f32;
use std::time::Duration;

use ggez::mint::Point2;

use crate::organisms::species::{Nutrition, Species};

#[derive(Clone)]
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
            energy: 0.0,
            health: species.max_health,
            species,
            age: Duration::ZERO,
        }
    }

    pub fn new_random(species: Species) -> Self {
        let age = Duration::from_secs_f32(rand::random::<f32>() * species.max_age.as_secs_f32());
        Self {
            position: Point2 { x: 0.0, y: 0.0 },
            energy: rand::random::<f32>() * species.max_energy,
            health: species.max_health,
            species,
            age,
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

    pub fn can_hunt(&self) -> bool {
        self.can_eat() && self.species.eats != Nutrition::None
    }

    pub fn energy(&self) -> f32 {
        self.energy
    }

    pub fn increase_energy(&mut self, amount: f32) {
        self.energy = f32::min(self.species.max_energy, self.energy + amount);
    }

    pub fn increase_age(&mut self, delta: Duration) {
        self.age += delta;
    }

    pub fn age(&self) -> Duration {
        self.age
    }
}
