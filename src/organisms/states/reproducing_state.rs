use super::organism_state::OrganismState;

pub struct ReproducingState {}

impl OrganismState for ReproducingState {
    fn initialize(_shared_state: &mut super::shared_state::SharedState) -> Self
    where
        Self: Sized,
    {
        Self {}
    }

    fn run(
        &mut self,
        _shared_state: &mut super::shared_state::SharedState,
        _delta: std::time::Duration,
    ) -> super::organism_state::StateTransition {
        todo!()
    }
}
