use std::time::Duration;

use ggez::mint::Point2;

use crate::{organisms::organism_result::OrganismResult, vector_helper};

use super::{
    eating_state::EatingState,
    organism_state::{AwarenessOfOtherOrganism, OrganismState, StateRunResult, StateTransition},
    shared_state::SharedState,
    walking_state::WalkingState,
};

const DISTANCE_TO_EAT: f32 = 0.1;
const SEEING_DISTANCE: f32 = 4.0;

pub struct HuntingState {
    awareness_of_others: Option<Vec<AwarenessOfOtherOrganism>>,
    hunted_organism_id_position: Option<(u64, Point2<f32>)>,
}
impl HuntingState {
    #[must_use]
    fn pick_new_target(&self, shared_state: &SharedState) -> Option<(u64, Point2<f32>)> {
        let awareness_of_others = self.awareness_of_others.as_ref().unwrap();
        let mut index_of_closest: Option<usize> = Option::None;
        let mut closest_position: Option<Point2<f32>> = Option::None;

        for (i, awareness_of_other) in awareness_of_others.iter().enumerate() {
            if awareness_of_other.species_name == shared_state.species.name
                || awareness_of_other.contains_nutrition != shared_state.species.eats
            {
                continue;
            }

            if !Self::is_close_enough(shared_state, awareness_of_other) {
                continue;
            }
            if index_of_closest.is_none() {
                index_of_closest = Some(i);
                closest_position = Some(awareness_of_other.position)
            } else if Self::is_closer_than(
                shared_state.position,
                awareness_of_other.position,
                closest_position.unwrap(),
            ) {
                index_of_closest = Some(i);
                closest_position = Some(awareness_of_other.position);
            }
        }

        if let Some(i) = index_of_closest {
            let closest: &AwarenessOfOtherOrganism = awareness_of_others.get(i).unwrap();
            Option::Some((closest.organism_id, closest.position))
        } else {
            Option::None
        }
    }

    fn is_close_enough(
        shared_state: &SharedState,
        awareness_of_other: &AwarenessOfOtherOrganism,
    ) -> bool {
        vector_helper::distance(shared_state.position, awareness_of_other.position)
            <= SEEING_DISTANCE
    }

    fn is_closer_than(
        my_position: Point2<f32>,
        this_new: Point2<f32>,
        than_current: Point2<f32>,
    ) -> bool {
        vector_helper::distance(my_position, this_new)
            < vector_helper::distance(my_position, than_current)
    }

    fn check_if_still_exists(id: u64, organisms_awareness: &[AwarenessOfOtherOrganism]) -> bool {
        organisms_awareness.iter().any(|x| x.organism_id == id)
    }
}

impl OrganismState for HuntingState {
    fn initialize(_shared_state: &mut super::shared_state::SharedState) -> Self
    where
        Self: Sized,
    {
        Self {
            awareness_of_others: Option::None,
            hunted_organism_id_position: Option::None,
        }
    }

    fn run(&mut self, shared_state: &mut SharedState, delta: Duration) -> StateRunResult {
        if self.hunted_organism_id_position.is_none() && self.awareness_of_others.is_none() {
            return StateRunResult::none_same();
        }

        if self.hunted_organism_id_position.is_none() {
            let new_target = self.pick_new_target(shared_state);
            if new_target.is_none() {
                return StateRunResult::none_next(WalkingState::init_boxed(shared_state));
            }
            self.hunted_organism_id_position = new_target;
        }

        if let Some((id, position)) = self.hunted_organism_id_position {
            if vector_helper::distance(shared_state.position, position) < DISTANCE_TO_EAT {
                if !Self::check_if_still_exists(id, self.awareness_of_others.as_ref().unwrap()) {
                    self.hunted_organism_id_position = self.pick_new_target(shared_state);
                    return StateRunResult::none_same();
                }
                StateRunResult {
                    organism_result: OrganismResult::AteOtherOrganism {
                        other_organism_id: id,
                    },
                    state_transition: StateTransition::Next(EatingState::init_boxed(shared_state)),
                }
            } else {
                shared_state.position = WalkingState::calculate_position(
                    delta,
                    shared_state.position,
                    position,
                    shared_state.species.walk_speed_s,
                );
                StateRunResult::none_same()
            }
        } else {
            StateRunResult::none_same()
        }
    }

    fn name(&self, _shared_state: &super::shared_state::SharedState) -> String {
        "hunting".into()
    }

    fn make_aware_of_others(&mut self, awareness_of_others: &Vec<AwarenessOfOtherOrganism>) {
        if self.awareness_of_others.is_some() {
            self.awareness_of_others.as_mut().unwrap().clear();
            self.awareness_of_others
                .as_mut()
                .unwrap()
                .clone_from(awareness_of_others);
        } else {
            self.awareness_of_others = Some(awareness_of_others.to_vec());
        }
    }

    fn init_boxed(shared_state: &mut super::shared_state::SharedState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self::initialize(shared_state))
    }
}
