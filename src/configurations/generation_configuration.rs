use super::species_generation_configuration::SpeciesGenerationConfiguration;

#[derive(Clone)]
pub struct GenerationConfiguration {
    pub species: Vec<SpeciesGenerationConfiguration>,
}