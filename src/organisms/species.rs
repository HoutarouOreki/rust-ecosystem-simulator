#[derive(Clone)]
pub struct Species {
    pub name: String,
    pub max_energy: u32,
    pub max_health: u32,
    pub max_age: u32,
    pub cost_of_birth: u32,
    pub can_walk: bool,
}