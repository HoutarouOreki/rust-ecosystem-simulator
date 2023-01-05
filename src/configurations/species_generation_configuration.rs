use crate::organisms::species::Species;

#[derive(Clone)]
pub struct SpeciesGenerationConfiguration {
    pub species: Species,
    pub amount_per_meter: f32,
}
