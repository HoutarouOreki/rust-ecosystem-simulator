use ggez::{
    context::Has,
    graphics::{Canvas, Color, DrawParam, GraphicsContext, Mesh, Text},
    mint::Point2,
};

use crate::organisms::{organism::Organism, species::Species};

pub struct Environment {
    organisms: Vec<Box<Organism>>,
    step: u64,
    pub offset: Point2<f32>,
}

impl Environment {
    pub fn simulate(&mut self, delta_s: f32) {
        for organism in self.organisms.iter_mut() {
            organism.simulate(delta_s);
        }
        self.step += 1;
    }

    pub fn new() -> Environment {
        let mut organisms = Vec::new();
        let species = Species {
            name: String::from("Test"),
            max_energy: 1000,
            max_health: 1000,
            max_age: 10000000,
            cost_of_birth: 0,
            can_walk: true,
        };
        let mut organism = Box::new(Organism::new(Box::new(species)));
        organism.set_position(Point2 { x: 400., y: 400. });
        organisms.push(organism);
        Environment {
            organisms,
            step: 0,
            offset: Point2 { x: 0., y: 0. },
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, gfx: &impl Has<GraphicsContext>) {
        self.draw_lines(canvas, gfx);

        for organism in self.organisms.iter() {
            let mut parent_absolute_rect = canvas.screen_coordinates().unwrap();
            parent_absolute_rect.translate(self.offset);
            organism.draw(&parent_absolute_rect, canvas, gfx);
        }

        canvas.draw(&Text::new(&self.step.to_string()), DrawParam::default())
    }

    fn draw_lines(&self, canvas: &mut Canvas, gfx: &impl Has<GraphicsContext>) {
        let color = Color::from_rgb(50, 50, 50);

        let distance = 50f32;
        let left = self.offset.x % distance - distance;
        let top = self.offset.y % distance - distance;

        let screen_rect = canvas.screen_coordinates().unwrap();

        let vertical_line = Mesh::new_line(
            gfx,
            &[
                Point2 {
                    x: 0.0,
                    y: screen_rect.top(),
                },
                Point2 {
                    x: 0.0,
                    y: screen_rect.bottom(),
                },
            ],
            1.0,
            color,
        ).unwrap();

        let horizontal_line = Mesh::new_line(
            gfx,
            &[
                Point2 {
                    x: screen_rect.left(),
                    y: 0.0,
                },
                Point2 {
                    x: screen_rect.right(),
                    y: 0.0,
                },
            ],
            1.0,
            color,
        ).unwrap();

        // vertical lines
        let mut current = left;
        while current < screen_rect.right() {
            current += distance;
            let draw_param = DrawParam::default().dest(Point2 { x: current, y: 0.0 });
            canvas.draw(&vertical_line, draw_param);
        }

        // horizontal lines
        let mut current = top;
        while current < screen_rect.bottom() {
            current += distance;
            let draw_param = DrawParam::default().dest(Point2 { x: 0.0, y: current });
            canvas.draw(&horizontal_line, draw_param);
        }
    }
}
