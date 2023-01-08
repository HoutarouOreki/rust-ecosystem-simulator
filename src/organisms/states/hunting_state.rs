use std::time::Duration;

use ggez::mint::Point2;

use crate::{organisms::organism_result::OrganismResult, vector_helper};

use super::{
    eating_state::EatingState,
    organism_state::{ForeignerInfo, OrganismState, StateRunResult, StateTransition},
    shared_state::SharedState,
    walking_state::WalkingState,
};

const DISTANCE_TO_EAT: f32 = 0.1;
const SEEING_DISTANCE: f32 = 4.0;

pub struct HuntingState {
    hunted_organism_id_position: Option<(u64, Point2<f32>)>,
}
impl HuntingState {
    #[must_use]
    fn pick_new_target(
        &self,
        shared_state: &SharedState,
        foreigners_info: &[ForeignerInfo],
    ) -> Option<(u64, Point2<f32>)> {
        let mut index_of_closest: Option<usize> = Option::None;
        let mut closest_position: Option<Point2<f32>> = Option::None;

        for (i, foreigner_info) in foreigners_info.iter().enumerate() {
            if foreigner_info.species_name == shared_state.species.name
                || foreigner_info.contains_nutrition != shared_state.species.eats
            {
                continue;
            }

            if !Self::is_close_enough(shared_state, foreigner_info) {
                continue;
            }
            if index_of_closest.is_none() {
                index_of_closest = Some(i);
                closest_position = Some(foreigner_info.position)
            } else if Self::is_closer_than(
                shared_state.position,
                foreigner_info.position,
                closest_position.unwrap(),
            ) {
                index_of_closest = Some(i);
                closest_position = Some(foreigner_info.position);
            }
        }

        if let Some(i) = index_of_closest {
            let closest: &ForeignerInfo = foreigners_info.get(i).unwrap();
            Option::Some((closest.organism_id, closest.position))
        } else {
            Option::None
        }
    }

    fn is_close_enough(shared_state: &SharedState, foreigner_info: &ForeignerInfo) -> bool {
        vector_helper::distance(shared_state.position, foreigner_info.position) <= SEEING_DISTANCE
    }

    fn is_closer_than(
        my_position: Point2<f32>,
        this_new: Point2<f32>,
        than_current: Point2<f32>,
    ) -> bool {
        vector_helper::distance(my_position, this_new)
            < vector_helper::distance(my_position, than_current)
    }

    fn check_if_still_exists(id: u64, foreigners_info: &[ForeignerInfo]) -> bool {
        foreigners_info.iter().any(|x| x.organism_id == id)
    }

    fn hunt_organism(
        &mut self,
        shared_state: &mut SharedState,
        hunted_position: Point2<f32>,
        hunted_id: u64,
        foreigners_info: &[ForeignerInfo],
        delta: Duration,
    ) -> StateRunResult {
        if vector_helper::distance(shared_state.position, hunted_position) < DISTANCE_TO_EAT {
            if !Self::check_if_still_exists(hunted_id, foreigners_info) {
                self.hunted_organism_id_position =
                    self.pick_new_target(shared_state, foreigners_info);
                return StateRunResult::none_same();
            }
            StateRunResult {
                organism_result: OrganismResult::AteOtherOrganism {
                    other_organism_id: hunted_id,
                },
                state_transition: StateTransition::Next(EatingState::init_boxed(shared_state)),
            }
        } else {
            calculate_and_set_position(shared_state, delta, hunted_position);
            StateRunResult::none_same()
        }
    }
}

impl OrganismState for HuntingState {
    fn initialize(_shared_state: &mut super::shared_state::SharedState) -> Self
    where
        Self: Sized,
    {
        Self {
            hunted_organism_id_position: Option::None,
        }
    }

    fn run(
        &mut self,
        shared_state: &mut SharedState,
        delta: Duration,
        foreigners_info: &[ForeignerInfo],
    ) -> StateRunResult {
        if self.hunted_organism_id_position.is_none() {
            let new_target = self.pick_new_target(shared_state, foreigners_info);
            if new_target.is_none() {
                return StateRunResult::none_next(WalkingState::init_boxed(shared_state));
            }
            self.hunted_organism_id_position = new_target;
        }

        if let Some((hunted_id, hunted_position)) = self.hunted_organism_id_position {
            self.hunt_organism(
                shared_state,
                hunted_position,
                hunted_id,
                foreigners_info,
                delta,
            )
        } else {
            StateRunResult::none_same()
        }
    }

    fn name(&self, _shared_state: &super::shared_state::SharedState) -> String {
        "hunting".into()
    }

    fn init_boxed(shared_state: &mut super::shared_state::SharedState) -> Box<Self>
    where
        Self: Sized,
    {
        Box::new(Self::initialize(shared_state))
    }
}

fn calculate_and_set_position(
    shared_state: &mut SharedState,
    delta: Duration,
    target_position: Point2<f32>,
) {
    shared_state.position = WalkingState::calculate_position(
        delta,
        shared_state.position,
        target_position,
        shared_state.species.walk_speed_s,
    );
}
