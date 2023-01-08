use std::time::Duration;

use ggez::mint::Point2;

use crate::{
    environment_awareness::EnvironmentAwareness, organisms::organism_result::OrganismResult,
    vector_helper,
};

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
        environment_awareness: &EnvironmentAwareness,
    ) -> Option<(u64, Point2<f32>)> {
        let mut closest: Option<ForeignerInfo> = Option::None;

        let foreigners_in_radius = get_foreigners_in_eyesight(environment_awareness, shared_state);
        for foreigner_info in foreigners_in_radius {
            if foreigner_info.species_name == shared_state.species.name
                || foreigner_info.contains_nutrition != shared_state.species.eats
            {
                continue;
            }

            if !Self::is_close_enough(shared_state, foreigner_info) {
                continue;
            }
            if closest.is_none()
                || Self::is_closer_than(
                    shared_state.position,
                    foreigner_info.position,
                    closest.to_owned().unwrap().position,
                )
            {
                closest = Some(foreigner_info.clone());
            }
        }

        if let Some(closest) = closest {
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

    fn check_if_still_exists(
        id: u64,
        environment_awareness: &EnvironmentAwareness,
        shared_state: &SharedState,
    ) -> bool {
        get_foreigners_in_eyesight(environment_awareness, shared_state).any(|x| x.organism_id == id)
    }

    fn hunt_organism(
        &mut self,
        shared_state: &mut SharedState,
        hunted_position: Point2<f32>,
        hunted_id: u64,
        environment_awareness: &EnvironmentAwareness,
        delta: Duration,
    ) -> StateRunResult {
        if vector_helper::distance(shared_state.position, hunted_position) < DISTANCE_TO_EAT {
            if !Self::check_if_still_exists(hunted_id, environment_awareness, shared_state) {
                self.hunted_organism_id_position =
                    self.pick_new_target(shared_state, environment_awareness);
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

fn get_foreigners_in_eyesight<'a>(
    environment_awareness: &'a EnvironmentAwareness,
    shared_state: &SharedState,
) -> impl Iterator<Item = &'a ForeignerInfo> {
    environment_awareness.get_radius_around(
        shared_state.position,
        shared_state.species.eyesight_distance,
    )
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
        environment_awareness: &EnvironmentAwareness,
    ) -> StateRunResult {
        if self.hunted_organism_id_position.is_none() {
            let new_target = self.pick_new_target(shared_state, environment_awareness);
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
                environment_awareness,
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
