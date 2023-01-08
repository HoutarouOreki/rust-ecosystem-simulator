use std::time::Duration;

use ggez::mint::Point2;

use crate::organisms::{organism::Organism, organism_result::OrganismResult, species::Nutrition};

use super::shared_state::SharedState;

pub trait OrganismState {
    fn initialize(shared_state: &mut SharedState) -> Self
    where
        Self: Sized;

    fn run(
        &mut self,
        shared_state: &mut SharedState,
        delta: Duration,
        awareness_of_others: &[AwarenessOfOtherOrganism],
    ) -> StateRunResult;

    fn init_boxed(shared_state: &mut SharedState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self::initialize(shared_state))
    }

    fn name(&self, shared_state: &SharedState) -> String;
}

pub enum StateTransition {
    Same,
    Next(Box<dyn OrganismState>),
}

pub struct StateRunResult {
    pub organism_result: OrganismResult,
    pub state_transition: StateTransition,
}

impl StateRunResult {
    pub fn new(organism_result: OrganismResult, state_transition: StateTransition) -> Self {
        Self {
            organism_result,
            state_transition,
        }
    }

    pub fn none_same() -> Self {
        Self {
            organism_result: OrganismResult::None,
            state_transition: StateTransition::Same,
        }
    }

    pub fn none_next(next_state: Box<dyn OrganismState>) -> Self {
        Self {
            organism_result: OrganismResult::None,
            state_transition: StateTransition::Next(next_state),
        }
    }
}

#[derive(Clone)]
pub struct AwarenessOfOtherOrganism {
    pub organism_id: u64,
    pub position: Point2<f32>,
    pub species_name: String,
    pub looks_for: Nutrition,
    pub contains_nutrition: Nutrition,
}
impl AwarenessOfOtherOrganism {
    pub fn new(organism: &Organism) -> AwarenessOfOtherOrganism {
        Self {
            organism_id: organism.id(),
            position: organism.position(),
            species_name: organism.shared_state().species.name.clone(),
            looks_for: organism.shared_state().species.eats,
            contains_nutrition: organism.shared_state().species.contained_nutrition,
        }
    }
}
