use std::sync::atomic::{AtomicU64, Ordering};

use ggez::{
    context::Has,
    graphics::{Canvas, Color, DrawMode, DrawParam, FillOptions, GraphicsContext, Mesh, Rect},
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
    age: u32,
    species: Box<Species>,
    layout_info: LayoutInfo,
    walking_manager: Option<WalkingManager>,
}

impl Organism {
    pub fn draw(
        &self,
        parent_absolute_rect: &Rect,
        canvas: &mut Canvas,
        gfx: &impl Has<GraphicsContext>,
    ) {
        let absolute_rect = self.layout_info.get_absolute_rect(parent_absolute_rect);
        let drawable: Mesh = Mesh::new_circle(
            gfx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Point2 { x: 0.0, y: 0.0 },
            14.0,
            0.4,
            Color::from_rgb(0, 91, 150),
        )
        .unwrap();

        let draw_param = DrawParam::default().dest(absolute_rect.point());

        canvas.draw(&drawable, draw_param)
    }

    pub fn eat(&mut self, amount: u32) {
        self.energy = std::cmp::min(self.species.max_energy, self.energy + amount);
    }

    pub fn id(&self) -> u64 {
        self.id
    }

    pub fn is_dead(&self) -> bool {
        self.age > self.species.max_age
    }

    pub fn new(species: Box<Species>) -> Self {
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
            age: 0,
            species: species.clone(),
            layout_info: LayoutInfo::new(),
            walking_manager: walking_manager,
        }
    }

    pub fn position(&self) -> Point2<f32> {
        self.position
    }

    pub fn simulate(&mut self, delta_s: f32) {
        assert!(!self.is_dead());

        self.try_reproduce();
        self.try_eat();
        self.walking(delta_s);

        self.layout_info = LayoutInfo {
            relative_rect: Rect {
                x: self.position.x,
                y: self.position.y,
                w: 20.0,
                h: 20.0,
            },
        };

        self.age += 1;
    }

    pub(crate) fn try_eat(&self) {
        //todo!()
    }

    pub(crate) fn try_reproduce(&self) {
        //todo!()
    }

    fn walking(&mut self, delta_s: f32) {
        if self.walking_manager.is_none() {
            return;
        }

        self.position = self
            .walking_manager
            .as_mut()
            .unwrap()
            .simulate_and_calculate_new_pos(self.position, delta_s);
    }

    pub fn set_position(&mut self, position: Point2<f32>) {
        self.position = position;
    }
}
