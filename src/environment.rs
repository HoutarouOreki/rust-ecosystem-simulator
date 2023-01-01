use ggez::{
    context::Has,
    graphics::{Canvas, Color, DrawParam, GraphicsContext, Mesh, Rect, Text},
    mint::Point2,
};

use crate::organisms::{organism::Organism, species::Species};

pub struct Environment {
    organisms: Vec<Organism>,
    step: u64,
    pub offset: Point2<i32>,
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
        let mut organism = Organism::new(Box::new(species));
        organism.set_position(Point2 { x: 400., y: 400. });
        organisms.push(organism);
        Environment {
            organisms,
            step: 0,
            offset: Point2 { x: 0, y: 0 },
        }
    }

    pub fn draw(&self, canvas: &mut Canvas, gfx: &impl Has<GraphicsContext>) {
        self.draw_lines(canvas, gfx);

        for organism in self.organisms.iter() {
            let mut parent_absolute_rect = canvas.screen_coordinates().unwrap();
            let offset = Point2 {
                x: self.offset.x as f32,
                y: self.offset.y as f32,
            };
            parent_absolute_rect.translate(offset);
            organism.draw(&parent_absolute_rect, canvas, gfx);
        }

        canvas.draw(&Text::new(self.step.to_string()), DrawParam::default())
    }

    fn draw_lines(&self, canvas: &mut Canvas, gfx: &impl Has<GraphicsContext>) {
        let color = Color::from_rgb(50, 50, 50);
        let distance = 50i32;

        let screen_rect = canvas.screen_coordinates().unwrap();

        let horizontal_start = self.offset.x % distance - distance;
        let vertical_start = self.offset.y % distance - distance;

        let (vertical_line, horizontal_line) =
            create_vertical_horizontal_lines(gfx, screen_rect, color);

        // vertical lines
        for line_x in (horizontal_start..=screen_rect.right() as i32).step_by(distance as usize) {
            let draw_param = DrawParam::default().dest(Point2 {
                x: line_x as f32,
                y: 0.0,
            });
            canvas.draw(&vertical_line, draw_param);
        }

        // horizontal lines
        for line_y in (vertical_start..=screen_rect.bottom() as i32).step_by(distance as usize) {
            let draw_param = DrawParam::default().dest(Point2 {
                x: 0.0,
                y: line_y as f32,
            });
            canvas.draw(&horizontal_line, draw_param);
        }
    }
}

fn create_vertical_horizontal_lines(
    gfx: &impl Has<GraphicsContext>,
    screen_rect: Rect,
    color: Color,
) -> (Mesh, Mesh) {
    let vertical_line = create_line(
        gfx,
        Point2 {
            x: 0.0,
            y: screen_rect.top(),
        },
        Point2 {
            x: 0.0,
            y: screen_rect.bottom(),
        },
        color,
    );
    let horizontal_line = create_line(
        gfx,
        Point2 {
            x: screen_rect.left(),
            y: 0.0,
        },
        Point2 {
            x: screen_rect.right(),
            y: 0.0,
        },
        color,
    );
    (vertical_line, horizontal_line)
}

fn create_line(
    gfx: &impl Has<GraphicsContext>,
    point_a: Point2<f32>,
    point_b: Point2<f32>,
    color: Color,
) -> Mesh {
    Mesh::new_line(gfx, &[point_a, point_b], 1.0, color).unwrap()
}
