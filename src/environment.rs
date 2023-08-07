use std::{
    collections::{hash_map::RandomState, HashMap},
    time::Duration,
};

use ggez::{
    context::Has,
    graphics::{
        Canvas, Color, DrawMode, DrawParam, FillOptions, GraphicsContext, InstanceArray, Mesh,
        Rect, Text,
    },
    input::keyboard::KeyboardContext,
    mint::Point2,
    winit::event::VirtualKeyCode,
    Context,
};

use crate::{
    application_context::ApplicationContext,
    configurations::generation_configuration::GenerationConfiguration, layout_info::LayoutInfo,
    simulation_thread::SimulationThread,
};

const CAMERA_SPEED: f32 = 400.0;
const ZOOM_SPEED: f32 = 1.4;

pub struct Environment {
    step: i64,
    time: Duration,
    offset: Point2<f32>,
    zoom: f32,
    circle_mesh: Option<Mesh>,
    layout_info: LayoutInfo,
    key_dictionary: HashMap<VirtualKeyCode, [f32; 2], RandomState>,
    organisms_mesh: InstanceArray,
    vertical_horizontal_lines: Option<(Mesh, Mesh)>,
    lines_horizontal_mesh: InstanceArray,
    lines_vertical_mesh: InstanceArray,
    simulate_every_n_organism: usize,
    cull_organisms_outside_view: bool,
    zoom_velocity: f32,
    simulation_thread: SimulationThread,
    time_step: Duration,
}

impl Environment {
    pub fn simulate(&mut self, mut steps: u32) {
        self.time += self.time_step * steps;
        self.step += steps as i64;
        while steps > 0 {
            self.simulation_thread.advance(self.time);
            steps -= 1;
        }
        self.simulation_thread.probe();
    }

    pub fn change_time_step(&mut self, time_step: Duration) {
        self.time_step = time_step;
        self.simulation_thread.change_time_step(time_step);
    }

    pub fn new(
        ctx: &Context,
        initial_time_step: Duration,
        generation_configuration: &GenerationConfiguration,
    ) -> Environment {
        let mut layout_info = LayoutInfo::new_centered();
        layout_info.relative_size = Point2 { x: true, y: true };

        let simulation_thread =
            SimulationThread::new(initial_time_step, generation_configuration.clone());

        Environment {
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
            organisms_mesh: InstanceArray::new(&ctx.gfx, Option::None),
            vertical_horizontal_lines: Option::None,
            lines_horizontal_mesh: InstanceArray::new(&ctx.gfx, Option::None),
            lines_vertical_mesh: InstanceArray::new(&ctx.gfx, Option::None),
            simulate_every_n_organism: 1,
            cull_organisms_outside_view: false,
            zoom_velocity: 0.0,
            simulation_thread,
            time_step: initial_time_step,
        }
    }

    pub fn draw(
        &mut self,
        canvas: &mut Canvas,
        gfx: &impl Has<GraphicsContext>,
        _application_context: &ApplicationContext,
    ) {
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
        // for organism in self.organisms.iter() {
        //     organism.draw(
        //         &environment_screen_rect,
        //         self.zoom,
        //         canvas,
        //         gfx,
        //         circle_mesh,
        //         application_context,
        //     );
        // }
        let visibility_rect = canvas.screen_coordinates().unwrap();

        self.organisms_mesh.set(
            self.simulation_thread
                .last_data
                .organisms
                .iter()
                .filter_map(|o| {
                    o.get_draw_param(&environment_screen_rect, self.zoom, &visibility_rect)
                }),
        );
        // canvas.draw(&self.organisms_mesh, DrawParam::default());
        canvas.draw_instanced_mesh(
            circle_mesh.to_owned(),
            &self.organisms_mesh,
            DrawParam::default(),
        );

        canvas.draw(
            &Text::new(format!(
                "simulated / requested / difference\n\
                {:.2} / {:.2} / {:.0}\n\
                {:.2} / {:.2} / {:.2}\n\n\
                organisms:{}\n\
                drawn:{}\n\n\
                {}\nnth organism: {}",
                self.simulation_thread.last_data.step,
                self.step,
                self.step - self.simulation_thread.last_data.step as i64,
                self.simulation_thread.last_data.time.as_secs_f32(),
                self.time.as_secs_f32(),
                self.time.as_secs_f32() - self.simulation_thread.last_data.time.as_secs_f32(),
                self.simulation_thread.last_data.organisms.len(),
                self.organisms_mesh.instances().len(),
                Self::species_count_string(&self.simulation_thread.last_data.organism_counter),
                self.simulate_every_n_organism,
            )),
            DrawParam::default(),
        );
    }

    fn species_count_string(organism_counter: &HashMap<String, u32>) -> String {
        let mut s = String::with_capacity(100);
        for (species_name, species_count) in organism_counter {
            s += &format!("{}: {}\n", species_name, species_count);
        }
        s
    }

    fn get_new_circle_mesh(gfx: &impl Has<GraphicsContext>) -> Mesh {
        let size = 10.0;
        Mesh::new_circle(
            gfx,
            DrawMode::Fill(FillOptions::DEFAULT),
            Point2 { x: 0.0, y: 0.0 },
            size * 0.5,
            0.07,
            Color::WHITE,
        )
        // Mesh::new_rectangle(
        //     gfx,
        //     DrawMode::Fill(FillOptions::DEFAULT),
        //     Rect {
        //         x: -size * 0.5,
        //         y: -size * 0.5,
        //         w: size,
        //         h: size,
        //     },
        //     Color::WHITE,
        // )
        .unwrap()
    }

    fn calculate_lines_distance(zoom: f32) -> f32 {
        let min_distance = 64.0;

        let mut distance = 0.0;
        while distance < min_distance {
            distance += zoom;
        }

        distance
    }

    fn calculate_first_line(env_boundary: f32, lines_distance: f32) -> f32 {
        if env_boundary > 0.0 {
            return env_boundary;
        }

        let skips = (-env_boundary / lines_distance).floor();

        env_boundary + lines_distance * skips
    }

    fn draw_lines(
        &mut self,
        canvas: &mut Canvas,
        parent_screen_rect: &Rect,
        environment_screen_rect: &Rect,
        gfx: &impl Has<GraphicsContext>,
    ) {
        let color = Color::from_rgb(30, 30, 30);

        let lines_distance = Self::calculate_lines_distance(self.zoom);
        let x_start = Self::calculate_first_line(environment_screen_rect.x, lines_distance);
        let y_start = Self::calculate_first_line(environment_screen_rect.y, lines_distance);

        let (vertical_line, horizontal_line) =
            self.vertical_horizontal_lines
                .get_or_insert(create_vertical_horizontal_lines(
                    gfx,
                    *environment_screen_rect,
                    color,
                ));

        self.lines_horizontal_mesh.clear();

        Self::recreate_lines_instance_mesh(
            &mut self.lines_horizontal_mesh,
            true,
            y_start,
            parent_screen_rect.bottom(),
            lines_distance,
        );

        canvas.draw_instanced_mesh(
            vertical_line.to_owned(),
            &self.lines_vertical_mesh,
            DrawParam::default(),
        );

        self.lines_vertical_mesh.clear();

        Self::recreate_lines_instance_mesh(
            &mut self.lines_vertical_mesh,
            false,
            x_start,
            parent_screen_rect.right(),
            lines_distance,
        );

        canvas.draw_instanced_mesh(
            horizontal_line.to_owned(),
            &self.lines_horizontal_mesh,
            DrawParam::default(),
        );
    }

    fn recreate_lines_instance_mesh(
        lines_mesh: &mut InstanceArray,
        horizontal: bool,
        start: f32,
        end: f32,
        lines_distance: f32,
    ) {
        let mut pos = start;
        while pos <= end {
            let mut draw_param = DrawParam::default().dest(Point2 { x: pos, y: 0.0 });
            if horizontal {
                draw_param = draw_param.dest(Point2 { x: 0.0, y: pos })
            }
            lines_mesh.push(draw_param);
            pos += lines_distance;
        }
    }

    pub fn handle_camera_controls(&mut self, ctx: &Context) {
        self.zoom += self.zoom * ZOOM_SPEED * self.zoom_velocity * ctx.time.delta().as_secs_f32();

        if self.zoom.is_nan() {
            self.zoom = 1.0;
        } else {
            self.zoom = self.zoom.clamp(0.01, 10000.0);
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

    pub fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) {
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::Plus | VirtualKeyCode::Equals if !_repeated => {
                    self.zoom_velocity += 1.0;
                }
                VirtualKeyCode::Minus | VirtualKeyCode::Underline if !_repeated => {
                    self.zoom_velocity -= 1.0;
                }
                VirtualKeyCode::PageDown => {
                    if self.simulate_every_n_organism > 1 {
                        self.simulate_every_n_organism -= 1;
                    }
                }
                VirtualKeyCode::PageUp => {
                    if self.simulate_every_n_organism < 32 {
                        self.simulate_every_n_organism += 1;
                    }
                }
                VirtualKeyCode::X => self.cull_organisms_outside_view = true,
                _ => {}
            }
        }
    }

    pub fn key_up_event(&mut self, _ctx: &mut Context, input: ggez::input::keyboard::KeyInput) {
        if let Some(keycode) = input.keycode {
            match keycode {
                VirtualKeyCode::Plus | VirtualKeyCode::Equals => {
                    self.zoom_velocity -= 1.0;
                }
                VirtualKeyCode::Minus | VirtualKeyCode::Underline => {
                    self.zoom_velocity += 1.0;
                }
                _ => {}
            }
        }
    }

    pub fn restart(&mut self, species_gen_config: GenerationConfiguration) {
        self.time = Duration::ZERO;
        self.step = 0;
        self.simulation_thread.restart(species_gen_config);
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
