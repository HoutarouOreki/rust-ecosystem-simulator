use serde::{Deserialize, Serialize};

use crate::organisms::species::Species;

#[derive(Clone, Serialize, Deserialize)]
pub struct SpeciesGenerationConfiguration {
    pub species: Species,
    pub amount_per_meter: f32,
}
