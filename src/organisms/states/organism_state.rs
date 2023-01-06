use std::time::Duration;

use crate::organisms::organism::Organism;

pub trait OrganismState {
    fn initialize(organism: &mut Organism) -> Self
    where
        Self: Sized;

    fn run(&mut self, organism: &mut Organism, delta: Duration) -> Box<dyn OrganismState>;
}
