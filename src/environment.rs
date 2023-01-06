use std::{
    collections::{hash_map::RandomState, HashMap},
    time::Duration,
};

use ggez::{
    context::Has,
    graphics::{
        Canvas, Color, DrawMode, DrawParam, FillOptions, GraphicsContext, Mesh, Rect, Text,
    },
    input::keyboard::KeyboardContext,
    mint::Point2,
    winit::event::VirtualKeyCode,
    Context,
};
use rand::{distributions::Uniform, prelude::Distribution};

use crate::{
    configurations::generation_configuration::GenerationConfiguration, layout_info::LayoutInfo,
    organisms::organism::Organism,
};

const BOUNDARY_DISTANCE_FROM_CENTER: f32 = 10f32;
const WORLD_SIZE: f32 =
    (2.0 * BOUNDARY_DISTANCE_FROM_CENTER) * (2.0 * BOUNDARY_DISTANCE_FROM_CENTER);

const CAMERA_SPEED: f32 = 200.0;
const ZOOM_SPEED: f32 = 1.4;

pub struct Environment {
    organisms: Vec<Organism>,
    step: u64,
    time: Duration,
    offset: Point2<f32>,
    zoom: f32,
    circle_mesh: Option<Mesh>,
    layout_info: LayoutInfo,
    key_dictionary: HashMap<VirtualKeyCode, [f32; 2], RandomState>,
}

impl Environment {
    pub fn simulate(&mut self, delta: Duration) {
        for organism in self.organisms.iter_mut() {
            organism.simulate(delta);
        }
        self.organisms.retain(|x| x.is_alive());
        self.step += 1;
        self.time += delta;
    }

    pub fn new(generation_configuration: &GenerationConfiguration) -> Environment {
        let organisms = Self::generate_organisms(generation_configuration);
        let mut layout_info = LayoutInfo::new_centered();
        layout_info.relative_size = Point2 { x: true, y: true };
        Environment {
            organisms,
            step: 0,
            offset: Point2 { x: 0., y: 0. },
            zoom: 100.0,
            circle_mesh: Option::None,
            time: Duration::ZERO,
            layout_info,
            key_dictionary: HashMap::from([
                (VirtualKeyCode::Left, [1f32, 0f32]),
                (VirtualKeyCode::Right, [-1f32, 0f32]),
                (VirtualKeyCode::Up, [0f32, 1f32]),
                (VirtualKeyCode::Down, [0f32, -1f32]),
            ]),
        }
    }

    fn generate_organisms(generation_configuration: &GenerationConfiguration) -> Vec<Organism> {
        let mut organisms = Vec::new();

        let mut rng = rand::thread_rng();
        let coordinate_uniform = Uniform::new_inclusive(
            -BOUNDARY_DISTANCE_FROM_CENTER,
            BOUNDARY_DISTANCE_FROM_CENTER,
        );

        for species_configuration in &generation_configuration.species {
            let organisms_amount = (species_configuration.amount_per_meter * WORLD_SIZE) as u32;

            for _ in 0..organisms_amount {
                let mut organism = Organism::new(species_configuration.species.to_owned());
                organism.set_position_x_y(
                    coordinate_uniform.sample(&mut rng),
                    coordinate_uniform.sample(&mut rng),
                );
                organisms.push(organism);
            }
        }

        organisms
    }

    pub fn draw(&mut self, canvas: &mut Canvas, gfx: &impl Has<GraphicsContext>) {
        let display_screen_rect = canvas.screen_coordinates().unwrap();

        let zoom_container = LayoutInfo {
            raw_rect_in_parent: Rect {
                x: 0.,
                y: 0.,
                w: 1.,
                h: 1.,
            },
            anchor: Point2 { x: 0.5, y: 0.5 },
            origin: Point2 { x: 0.5, y: 0.5 },
            scale: Point2 { x: 1.0, y: 1.0 },
            relative_size: Point2 { x: true, y: true },
        };

        let zoom_container_screen_rect = zoom_container.get_screen_rect(&display_screen_rect, 1.0);

        let environment_screen_rect = self
            .layout_info
            .get_screen_rect(&zoom_container_screen_rect, self.zoom);

        self.draw_lines(canvas, &display_screen_rect, &environment_screen_rect, gfx);

        let circle_mesh = self
            .circle_mesh
            .get_or_insert(Self::get_new_circle_mesh(gfx));
        for organism in self.organisms.iter() {
            organism.draw(
                &environment_screen_rect,
                self.zoom,
                canvas,
                gfx,
                circle_mesh,
            );
        }

        canvas.draw(&Text::new(self.step.to_string()), DrawParam::default());
        canvas.draw(
            &Text::new(format!("{:.2}", self.time.as_secs_f32())),
            DrawParam::default().dest([0.0, 20.0]),
        )
    }

    fn get_new_circle_mesh(gfx: &impl Has<GraphicsContext>) -> Mesh {
        Mesh::new_circle(
            gfx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Point2 { x: 0.0, y: 0.0 },
            0.5,
            0.01,
            Color::WHITE,
        )
        .unwrap()
    }

    fn calculate_first_line(&self, env_boundary: f32) -> f32 {
        if env_boundary > 0.0 {
            return env_boundary;
        }

        let skips = (env_boundary / self.zoom).abs().floor();
        env_boundary + self.zoom * skips
    }

    fn draw_lines(
        &self,
        canvas: &mut Canvas,
        parent_screen_rect: &Rect,
        environment_screen_rect: &Rect,
        gfx: &impl Has<GraphicsContext>,
    ) {
        let color = Color::from_rgb(50, 50, 50);

        let horizontal_start = self.calculate_first_line(environment_screen_rect.x);
        let vertical_start = self.calculate_first_line(environment_screen_rect.y);

        let (vertical_line, horizontal_line) =
            create_vertical_horizontal_lines(gfx, *environment_screen_rect, color);

        // vertical lines
        let mut line_x = horizontal_start;
        while line_x <= parent_screen_rect.right() {
            let draw_param = DrawParam::default().dest(Point2 { x: line_x, y: 0.0 });
            canvas.draw(&vertical_line, draw_param);
            line_x += self.zoom;
        }

        // horizontal lines
        let mut line_y = vertical_start;
        while line_y <= parent_screen_rect.bottom() {
            let draw_param = DrawParam::default().dest(Point2 { x: 0.0, y: line_y });
            canvas.draw(&horizontal_line, draw_param);
            line_y += self.zoom;
        }
    }

    pub fn handle_camera_controls(&mut self, ctx: &Context) {
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Plus) {
            self.zoom += self.zoom * ZOOM_SPEED * ctx.time.delta().as_secs_f32();
        }
        if ctx.keyboard.is_key_pressed(VirtualKeyCode::Minus) {
            self.zoom -= self.zoom * ZOOM_SPEED * ctx.time.delta().as_secs_f32();
        }

        if self.zoom.is_nan() {
            self.zoom = 1.0;
        } else {
            self.zoom = self.zoom.clamp(1.0, 10000.0);
        }

        let camera_moving_direction = self.direction_from_keyboard_state(&ctx.keyboard);

        if camera_moving_direction == [0f32, 0f32] {
            return;
        }

        let offset: [f32; 2] = vecmath::vec2_scale(
            vecmath::vec2_normalized(camera_moving_direction),
            CAMERA_SPEED * ctx.time.delta().as_secs_f32() / self.zoom,
        );

        self.layout_info.raw_rect_in_parent.translate(offset);
    }

    fn direction_from_keyboard_state(&self, ctx: &KeyboardContext) -> [f32; 2] {
        let mut direction = [0f32, 0f32];
        for (key, vector) in &self.key_dictionary {
            if ctx.is_key_pressed(*key) {
                direction = vecmath::vec2_add(direction, *vector);
            }
        }
        direction
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
