#![allow(dead_code)]

mod configurations;
mod environment;
mod layout_info;
mod organisms;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;
use std::time::Duration;

use configurations::generation_configuration::GenerationConfiguration;
use configurations::species_generation_configuration::SpeciesGenerationConfiguration;
use environment::Environment;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};

use ggez::input::keyboard::KeyboardContext;

use ggez::mint::Point2;
use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use organisms::species::Species;

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "HoutarouOreki")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);
    ctx.gfx.set_window_title("Ecosystem Simulator");
    let _resize_result = ctx.gfx.set_resizable(true);

    // Run!
    event::run(ctx, event_loop, my_game);
}

const CAMERA_SPEED: f32 = 200.0;

struct MyGame {
    time_since_last_simulation_step: Duration,
    time_per_step: Duration,
    environment: Environment,
    key_dictionary: HashMap<VirtualKeyCode, [f32; 2], RandomState>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let generation_configuration = generate_default_generation_configuration();
        MyGame {
            time_since_last_simulation_step: Duration::ZERO,
            environment: Environment::new(&generation_configuration),
            time_per_step: Duration::from_secs_f32(0.05),
            key_dictionary: HashMap::from([
                (VirtualKeyCode::Left, [1f32, 0f32]),
                (VirtualKeyCode::Right, [-1f32, 0f32]),
                (VirtualKeyCode::Up, [0f32, 1f32]),
                (VirtualKeyCode::Down, [0f32, -1f32]),
            ]),
        }
    }

    fn move_view(&mut self, ctx: &Context) {
        let camera_moving_direction = self.direction_from_keyboard_state(&ctx.keyboard);

        if camera_moving_direction == [0f32, 0f32] {
            return;
        }

        let offset: [f32; 2] = vecmath::vec2_add(
            [
                self.environment.offset.x as f32,
                self.environment.offset.y as f32,
            ],
            vecmath::vec2_scale(
                vecmath::vec2_normalized(camera_moving_direction),
                CAMERA_SPEED * ctx.time.delta().as_secs_f32(),
            ),
        );

        self.environment.offset = Point2 {
            x: offset[0] as i32,
            y: offset[1] as i32,
        };
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

fn generate_default_generation_configuration() -> GenerationConfiguration {
    GenerationConfiguration {
        species: vec![
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Test Species 1"),
                    max_energy: 256,
                    max_health: 256,
                    max_age: Duration::from_secs(200),
                    cost_of_birth: 60,
                    can_walk: true,
                    can_eat_organisms: true,
                    can_photosynthesize: false,
                    color: Color::from_rgb(0, 91, 150),
                },
                amount_per_meter: 0.2,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Test Species 2"),
                    max_energy: 30,
                    max_health: 30,
                    max_age: Duration::from_secs(9000),
                    cost_of_birth: 20,
                    can_walk: false,
                    can_eat_organisms: false,
                    can_photosynthesize: true,
                    color: Color::from_rgb(79, 121, 66),
                },
                amount_per_meter: 0.7,
            },
        ],
    }
}

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.time_since_last_simulation_step += ctx.time.delta();

        if self.time_since_last_simulation_step > self.time_per_step {
            self.time_since_last_simulation_step -= self.time_per_step;
            self.environment.simulate(self.time_per_step);
        }

        self.move_view(ctx);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        self.environment.draw(&mut canvas, ctx);

        canvas.finish(ctx)
    }
}
