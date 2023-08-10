use std::{
    collections::{HashMap, HashSet},
    time::Duration,
};

use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    configurations::generation_configuration::GenerationConfiguration,
    environment_awareness::EnvironmentAwareness,
    organisms::{organism::Organism, organism_info::OrganismInfo, organism_result::OrganismResult},
    simulation_thread::SimulationData,
    vector_helper,
};

const BOUNDARY_DISTANCE_FROM_CENTER: f32 = 100f32;
const WORLD_SIZE: f32 =
    (2.0 * BOUNDARY_DISTANCE_FROM_CENTER) * (2.0 * BOUNDARY_DISTANCE_FROM_CENTER);

pub struct Simulation {
    organisms: Vec<Organism>,
    to_add: Vec<Organism>,
    to_remove: HashSet<u64>,
    environment_awareness: EnvironmentAwareness,
    cull_organisms_outside_view: bool,
    pub simulation_data: SimulationData,
}

impl Simulation {
    pub fn new(generation_configuration: &GenerationConfiguration) -> Self {
        let organisms = Self::generate_organisms(generation_configuration);
        let mut organism_counter = HashMap::new();
        for organism in organisms.iter() {
            Self::adjust_species_counter(organism, &mut organism_counter, true, 1);
        }
        let organism_infos = OrganismInfo::new_from_organisms(&organisms);

        Simulation {
            organisms,
            to_add: Vec::new(),
            to_remove: HashSet::new(),
            environment_awareness: EnvironmentAwareness::new(32.0),
            cull_organisms_outside_view: false,
            simulation_data: SimulationData {
                organism_infos,
                organism_counter,
                time: Duration::ZERO,
                step: 0,
            },
        }
    }

    pub fn run(&mut self, delta: Duration) {
        self.environment_awareness.refill(&self.organisms);
        for organism in self.organisms.iter_mut() {
            match Self::simulate_organism(organism, delta, &self.environment_awareness) {
                OrganismsChange::Add(mut vec) => {
                    vec.iter().for_each(|x| {
                        Self::adjust_species_counter(
                            x,
                            &mut self.simulation_data.organism_counter,
                            true,
                            1,
                        )
                    });
                    self.to_add.append(&mut vec);
                }
                OrganismsChange::Remove(id) => {
                    self.to_remove.insert(id);
                }
                OrganismsChange::AddRemove(mut vec, id) => {
                    vec.iter().for_each(|x| {
                        Self::adjust_species_counter(
                            x,
                            &mut self.simulation_data.organism_counter,
                            true,
                            1,
                        )
                    });
                    self.to_add.append(&mut vec);
                    self.to_remove.insert(id);
                }
                OrganismsChange::None => {}
            };
        }
        self.organisms.retain(|x| {
            if !self.to_remove.contains(&x.id()) {
                true
            } else {
                Self::adjust_species_counter(
                    x,
                    &mut self.simulation_data.organism_counter,
                    false,
                    1,
                );
                false
            }
        });
        self.organisms.append(&mut self.to_add);
        self.simulation_data.step += 1;
        self.simulation_data.time += delta;
        self.cull_organisms_outside_view = false;

        OrganismInfo::from_organisms_fill_vec(
            &self.organisms,
            &mut self.simulation_data.organism_infos,
        );
    }

    fn simulate_organism(
        organism: &mut Organism,
        delta: Duration,
        environment_awareness: &EnvironmentAwareness,
    ) -> OrganismsChange {
        let result = organism.simulate(delta, environment_awareness);
        match result {
            OrganismResult::HadChildren { amount }
                if Self::can_add_children(organism, environment_awareness) =>
            {
                let vec = Self::create_organism_children(amount, organism);
                OrganismsChange::Add(vec)
            }
            OrganismResult::HadChildren { amount: _ } => OrganismsChange::None,
            OrganismResult::AteOtherOrganism { other_organism_id } => {
                OrganismsChange::Remove(other_organism_id)
            }
            OrganismResult::None => OrganismsChange::None,
            OrganismResult::Died => {
                OrganismsChange::AddRemove(vec![Organism::new_corpse(organism)], organism.id())
            }
            OrganismResult::Disappeared => OrganismsChange::Remove(organism.id()),
        }
    }

    fn can_add_children(organism: &Organism, environment_awareness: &EnvironmentAwareness) -> bool {
        let checked_distance = organism.shared_state().species.birth_distance * 1.0;
        let max_amount_others_of_same_species =
            organism.shared_state().species.max_per_meter * checked_distance * checked_distance;

        if max_amount_others_of_same_species == 0.0 {
            return true;
        }

        let others = environment_awareness.get_radius_around(organism.position(), checked_distance);
        let others_of_same_species =
            others.filter(|x| x.species_name == organism.shared_state().species.name);
        let amount_others_of_same_species = others_of_same_species.count() as f32;

        amount_others_of_same_species < max_amount_others_of_same_species
    }

    fn create_organism_children(amount: u64, organism: &Organism) -> Vec<Organism> {
        let mut vec = Vec::new();

        let angle = rand::random::<f32>() * std::f32::consts::TAU;
        let angle_increase = std::f32::consts::TAU / amount as f32;
        for i in 0..amount {
            let away_vector =
                vector_helper::create_direction_vector(angle + (angle_increase * i as f32));
            let child = Organism::new_child_away(organism, away_vector);
            vec.push(child);
        }
        vec
    }

    fn adjust_species_counter(
        organism: &Organism,
        organism_counter: &mut HashMap<String, u32>,
        increase: bool,
        amount: u32,
    ) {
        let species_name = organism.shared_state().clone().species.name;
        let species_count = organism_counter.get_mut(&organism.shared_state().clone().species.name);
        if let Some(count) = species_count {
            if increase {
                *count += amount;
            } else {
                *count -= amount;
            }
        } else {
            organism_counter.insert(species_name, 1);
        }
    }

    fn generate_organisms(generation_configuration: &GenerationConfiguration) -> Vec<Organism> {
        let mut organisms = Vec::new();

        let amount_multiplier = 0.1f32;

        let mut rng = rand::thread_rng();
        let coordinate_uniform = Uniform::new_inclusive(
            -BOUNDARY_DISTANCE_FROM_CENTER,
            BOUNDARY_DISTANCE_FROM_CENTER,
        );

        for species_configuration in &generation_configuration.species {
            let organisms_amount =
                (species_configuration.amount_per_meter * WORLD_SIZE * amount_multiplier) as u32;

            for _ in 0..organisms_amount {
                let mut organism =
                    Organism::new_randomized(species_configuration.species.to_owned());
                organism.set_position_x_y(
                    coordinate_uniform.sample(&mut rng),
                    coordinate_uniform.sample(&mut rng),
                );
                organisms.push(organism);
            }
        }

        organisms
    }
}

pub enum OrganismsChange {
    Add(Vec<Organism>),
    Remove(u64),
    AddRemove(Vec<Organism>, u64),
    None,
}
