use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use ggez::{
    context::Has,
    graphics::{Canvas, Color, DrawParam, GraphicsContext, Mesh, Rect},
    mint::Point2,
};

use crate::{
    environment_awareness::EnvironmentAwareness, layout_info::LayoutInfo,
    organisms::states::organism_state::StateTransition,
};

use super::{
    organism_result::OrganismResult,
    species::{HuntingBehavior, Nutrition, Species},
    states::{
        dead_state::DeadState, idle_state::IdleState, organism_state::OrganismState,
        shared_state::SharedState,
    },
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct Organism {
    pub id: u64,
    pub layout_info: LayoutInfo,
    state: Box<dyn OrganismState>,
    pub shared_state: SharedState,
}

impl Organism {
    pub fn draw(
        &self,
        parent_screen_rect: &Rect,
        parent_rect_scale: f32,
        canvas: &mut Canvas,
        _gfx: &impl Has<GraphicsContext>,
        circle_mesh: &Mesh,
    ) {
        let draw_param = self.get_draw_param(
            parent_screen_rect,
            parent_rect_scale,
            &canvas.screen_coordinates().unwrap(),
        );

        if draw_param.is_none() {
            return;
        }

        canvas.draw(circle_mesh, draw_param.unwrap());
    }

    pub fn get_draw_param(
        &self,
        parent_screen_rect: &Rect,
        parent_rect_scale: f32,
        visibility_rect: &Rect,
    ) -> Option<DrawParam> {
        let screen_rect = self
            .layout_info
            .get_screen_rect(parent_screen_rect, parent_rect_scale);

        if !screen_rect.overlaps(visibility_rect) {
            return Option::None;
        }

        Some(
            DrawParam::default()
                .dest_rect(screen_rect)
                .color(self.shared_state.species.color),
        )
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        !self.is_dead()
    }

    pub fn is_dead(&self) -> bool {
        self.shared_state.age() > self.shared_state.species.max_age
            && self.shared_state.health > 0.0
    }

    pub fn new_child(organism: &Organism) -> Self {
        let mut new_child = Organism::new(organism.shared_state.species.clone());
        new_child.set_position(organism.shared_state.position);
        new_child
    }

    pub fn new_child_away(organism: &Organism, away_vector: [f32; 2]) -> Self {
        let away_vector =
            vecmath::vec2_scale(away_vector, organism.shared_state.species.birth_distance);
        let mut new_child = Organism::new(organism.shared_state.species.clone());
        new_child.set_position(
            vecmath::vec2_add(organism.shared_state.position.into(), away_vector).into(),
        );
        new_child
    }

    pub fn new_randomized(species: Species) -> Self {
        let mut s = Self::new(species.clone());
        s.shared_state = SharedState::new_random(species);
        s
    }

    pub fn new(species: Species) -> Self {
        NEXT_ID.fetch_add(1, Ordering::SeqCst);

        let mut layout_info = LayoutInfo::new();
        layout_info.raw_rect_in_parent.w = 0.5;
        layout_info.raw_rect_in_parent.h = 0.5;

        let shared_state = SharedState::new_default(species);

        Self {
            id: NEXT_ID.load(Ordering::SeqCst),
            layout_info,
            shared_state,
            state: Box::new(IdleState::new()),
        }
    }

    pub fn new_corpse(organism: &Organism) -> Self {
        let mut s = Self::new(Species {
            name: String::from("Corpse"),
            max_energy: 50.0,
            max_health: 100.0, // so that it doesn't "die"
            max_age: Duration::from_secs(120),
            energy_cost_of_birth: 1.0,
            health_cost_of_birth: 0.0,
            walk_speed_s: 0.0,
            photosynthesis_rate_s: 0.0,
            color: Color::from_rgb(100, 100, 100),
            eats: Nutrition::None,
            contained_nutrition: Nutrition::Corpse,
            eyesight_distance: 0.0,
            birth_distance: 1.0,
            birth_immunity: Duration::ZERO,
            eating_distance: 0.2,
            max_per_meter: 0.0,
            hunting_behavior: HuntingBehavior::Closest,
        });
        s.shared_state.position = organism.position();
        s.state = DeadState::new_boxed();
        s
    }

    pub fn position(&self) -> Point2<f32> {
        self.shared_state.position
    }

    pub fn shared_state(&self) -> &SharedState {
        &self.shared_state
    }

    pub fn simulate(
        &mut self,
        delta: Duration,
        environment_awareness: &EnvironmentAwareness,
    ) -> OrganismResult {
        if self.is_dead() {
            if self.shared_state.species.contained_nutrition == Nutrition::Corpse
                || self.shared_state.species.contained_nutrition == Nutrition::Plant
            {
                return OrganismResult::Disappeared;
            } else {
                return OrganismResult::Died;
            }
        }

        self.shared_state
            .increase_energy(self.shared_state.species.photosynthesis_rate_s * delta.as_secs_f32());

        let state_run_result = self
            .state
            .run(&mut self.shared_state, delta, environment_awareness);
        if let StateTransition::Next(next_state) = state_run_result.state_transition {
            self.state = next_state;
        }

        self.layout_info = LayoutInfo {
            raw_rect_in_parent: Rect {
                x: self.shared_state.position.x,
                y: self.shared_state.position.y,
                w: 0.3,
                h: 0.3,
            },
            anchor: Point2 { x: 0.5, y: 0.5 },
            origin: Point2 { x: 0.5, y: 0.5 },
            scale: Point2 { x: 1.0, y: 1.0 },
            relative_size: Point2 { x: false, y: false },
        };

        self.shared_state.increase_age(delta);

        state_run_result.organism_result
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.shared_state.position = position;
    }

    pub fn set_position_x_y(&mut self, x: f32, y: f32) {
        self.shared_state.position = Point2 { x, y };
    }

    pub fn get_info_text(state: &dyn OrganismState, shared_state: &SharedState) -> String {
        let s = format!(
            "{}\r\nage: {}/{}",
            state.name(shared_state),
            shared_state.age().as_secs(),
            shared_state.species.max_age.as_secs()
        );

        s
    }
}
