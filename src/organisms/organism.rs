use std::{
    sync::atomic::{AtomicU64, Ordering},
    time::Duration,
};

use ggez::{
    context::Has,
    graphics::{Canvas, DrawParam, GraphicsContext, Mesh, Rect},
    mint::Point2,
};

use crate::{layout_info::LayoutInfo, organisms::states::organism_state::StateTransition};

use super::{
    species::Species,
    states::{idle_state::IdleState, organism_state::OrganismState, shared_state::SharedState},
};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct Organism {
    id: u64,
    age: Duration,
    layout_info: LayoutInfo,
    state: Box<dyn OrganismState>,
    shared_state: SharedState,
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
        let screen_rect = self
            .layout_info
            .get_screen_rect(parent_screen_rect, parent_rect_scale);

        let draw_param = DrawParam::default()
            .dest_rect(screen_rect)
            .color(self.shared_state.species.color);

        canvas.draw(circle_mesh, draw_param)
    }

    pub fn eat(&mut self, amount: u32) {
        self.shared_state.energy = std::cmp::min(
            self.shared_state.species.max_energy,
            self.shared_state.energy + amount,
        );
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        self.age <= self.shared_state.species.max_age
    }

    pub fn is_dead(&self) -> bool {
        self.age > self.shared_state.species.max_age
    }

    pub fn new(species: Species) -> Self {
        NEXT_ID.fetch_add(1, Ordering::SeqCst);

        let mut layout_info = LayoutInfo::new();
        layout_info.raw_rect_in_parent.w = 0.5;
        layout_info.raw_rect_in_parent.h = 0.5;

        let shared_state = SharedState {
            position: Point2 { x: 0.0, y: 0.0 },
            energy: species.max_energy,
            health: species.max_health,
            species,
        };

        Self {
            id: NEXT_ID.load(Ordering::SeqCst),
            age: Duration::ZERO,
            layout_info,
            shared_state,
            state: Box::new(IdleState::new()),
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.shared_state.position
    }

    pub fn simulate(&mut self, delta: Duration) {
        assert!(!self.is_dead());

        if let StateTransition::Next(next_state) = self.state.run(&mut self.shared_state, delta) {
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

        self.age += delta;
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.shared_state.position = position;
    }

    pub fn set_position_x_y(&mut self, x: f32, y: f32) {
        self.shared_state.position = Point2 { x, y };
    }
}
