use ggez::mint::Point2;

use crate::organisms::species::Species;

pub struct SharedState {
    pub position: Point2<f32>,
    pub energy: u32,
    pub health: u32,
    pub species: Species,
}
