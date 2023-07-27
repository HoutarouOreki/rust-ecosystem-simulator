use serde::{Serialize, Deserialize};

use super::species_generation_configuration::SpeciesGenerationConfiguration;

#[derive(Clone, Serialize, Deserialize)]
pub struct GenerationConfiguration {
    pub species: Vec<SpeciesGenerationConfiguration>,
}
