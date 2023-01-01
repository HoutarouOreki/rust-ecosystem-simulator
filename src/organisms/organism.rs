use std::{sync::atomic::{AtomicU64, Ordering}, time::Duration};

use ggez::{
    context::Has,
    graphics::{Canvas, DrawParam, GraphicsContext, Mesh, Rect},
    mint::Point2,
};

use crate::layout_info::LayoutInfo;

use super::{species::Species, walking_manager::WalkingManager};

static NEXT_ID: AtomicU64 = AtomicU64::new(0);

pub struct Organism {
    id: u64,
    position: Point2<f32>,
    energy: u32,
    health: u32,
    age: Duration,
    species: Species,
    layout_info: LayoutInfo,
    walking_manager: Option<WalkingManager>,
}

impl Organism {
    pub fn draw(
        &self,
        parent_absolute_rect: &Rect,
        parent_rect_scale: f32,
        canvas: &mut Canvas,
        _gfx: &impl Has<GraphicsContext>,
        circle_mesh: &Mesh,
    ) {
        let absolute_rect = self.layout_info.get_absolute_rect(parent_absolute_rect, parent_rect_scale);

        let draw_param = DrawParam::default().dest(absolute_rect.point()).color(self.species.color);

        canvas.draw(circle_mesh, draw_param)
    }

    pub fn eat(&mut self, amount: u32) {
        self.energy = std::cmp::min(self.species.max_energy, self.energy + amount);
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_alive(&self) -> bool {
        self.age <= self.species.max_age
    }

    pub fn is_dead(&self) -> bool {
        self.age > self.species.max_age
    }

    pub fn new(species: Species) -> Self {
        let walking_manager = if species.can_walk {
            Option::Some(WalkingManager::new())
        } else {
            Option::None
        };

        NEXT_ID.fetch_add(1, Ordering::SeqCst);
        Self {
            id: NEXT_ID.load(Ordering::SeqCst),
            position: Point2 { x: 0.0, y: 0.0 },
            energy: species.max_energy,
            health: species.max_health,
            age: Duration::ZERO,
            species,
            layout_info: LayoutInfo::new(),
            walking_manager,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn simulate(&mut self, delta: Duration) {
        assert!(!self.is_dead());

        self.try_reproduce();
        self.try_eat();
        self.walking(delta);

        self.layout_info = LayoutInfo {
            relative_rect: Rect {
                x: self.position.x,
                y: self.position.y,
                w: 20.0,
                h: 20.0,
            },
        };

        self.age += delta;
    }

    pub(crate) fn try_eat(&self) {
        //todo!()
    }

    pub(crate) fn try_reproduce(&self) {
        //todo!()
    }

    fn walking(&mut self, delta: Duration) {
        if self.walking_manager.is_none() {
            return;
        }

        self.position = self
            .walking_manager
            .as_mut()
            .unwrap()
            .simulate_and_calculate_new_pos(self.position, delta);
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }

    pub fn set_position_x_y(&mut self, x: f32, y: f32) {
        self.position = Point2 { x, y };
    }
}
