#![allow(dead_code)]

pub mod application_context;
mod configurations;
mod environment;
mod environment_awareness;
mod layout_info;
mod organisms;
pub mod vector_helper;

use std::time::Duration;

use application_context::ApplicationContext;
use configurations::generation_configuration::GenerationConfiguration;
use configurations::species_generation_configuration::SpeciesGenerationConfiguration;
use environment::Environment;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, BlendMode, Color, DrawParam, Text};

use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use organisms::species::{Nutrition, Species};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "HoutarouOreki")
        .window_setup(WindowSetup::default().samples(ggez::conf::NumSamples::Four))
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

struct MyGame {
    time_to_simulate: Duration,
    time_per_step: Duration,
    environment: Environment,
    application_context: ApplicationContext,
    speed: u32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let generation_configuration = generate_default_generation_configuration();
        MyGame {
            time_to_simulate: Duration::ZERO,
            environment: Environment::new(ctx, &generation_configuration),
            time_per_step: Duration::from_secs_f32(0.05),
            application_context: ApplicationContext::default(),
            speed: 1,
        }
    }
}

fn generate_default_generation_configuration() -> GenerationConfiguration {
    GenerationConfiguration {
        species: vec![
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Herbivore"),
                    max_energy: 256.0,
                    max_health: 256.0,
                    max_age: Duration::from_secs(60),
                    cost_of_birth: 40.0,
                    walk_speed_s: 0.6,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(0, 91, 150),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Plant,
                    eyesight_distance: 7.0,
                    birth_distance: 0.3,
                },
                amount_per_meter: 0.2,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Plant"),
                    max_energy: 150.0,
                    max_health: 30.0,
                    max_age: Duration::from_secs(50),
                    cost_of_birth: 50.0,
                    walk_speed_s: 0.0,
                    photosynthesis_rate_s: 2.0,
                    color: Color::from_rgb(10, 70, 10),
                    contained_nutrition: Nutrition::Plant,
                    eats: Nutrition::None,
                    eyesight_distance: 0.0,
                    birth_distance: 15.0,
                },
                amount_per_meter: 0.6,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Carnivore"),
                    max_energy: 120.0,
                    max_health: 30.0,
                    max_age: Duration::from_secs(35),
                    cost_of_birth: 80.0,
                    walk_speed_s: 3.9,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(200, 0, 0),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Meat,
                    eyesight_distance: 10.0,
                    birth_distance: 0.1,
                },
                amount_per_meter: 0.04,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Scavenger"),
                    max_energy: 150.0,
                    max_health: 30.0,
                    max_age: Duration::from_secs(150),
                    cost_of_birth: 101.0,
                    walk_speed_s: 14.2,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(100, 0, 150),
                    contained_nutrition: Nutrition::None,
                    eats: Nutrition::Corpse,
                    eyesight_distance: 30.0,
                    birth_distance: 0.7,
                },
                amount_per_meter: 0.01,
            },
        ],
    }
}

impl EventHandler for MyGame {
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
        _repeated: bool,
    ) -> GameResult {
        let time_per_step_step = Duration::from_secs_f32(0.005);
        match input.keycode {
            Some(VirtualKeyCode::H) => {
                self.application_context.draw_each_organism_info =
                    !self.application_context.draw_each_organism_info
            }
            Some(VirtualKeyCode::Semicolon) => {
                if let Some(new_value) = self.speed.checked_sub(1) {
                    self.speed = new_value;
                }
            }
            Some(VirtualKeyCode::Apostrophe) => self.speed += 1,
            Some(VirtualKeyCode::Period) => self.time_per_step += time_per_step_step,
            Some(VirtualKeyCode::Comma) => {
                if let Some(new_value) = self.time_per_step.checked_sub(time_per_step_step) {
                    self.time_per_step = new_value.max(time_per_step_step);
                }
            }
            _ => {}
        };
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.time_to_simulate += ctx.time.delta() * self.speed;

        if self.time_to_simulate > self.time_per_step {
            self.time_to_simulate -= self.time_per_step;
            self.environment
                .simulate(self.time_per_step, &self.application_context);
        }

        self.environment.handle_camera_controls(ctx);
        self.speed = self.speed.clamp(0, 32);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.set_blend_mode(BlendMode::REPLACE);
        canvas.set_premultiplied_text(false);

        self.environment
            .draw(&mut canvas, ctx, &self.application_context);

        let (width, _) = ctx.gfx.drawable_size();

        canvas.draw(
            &Text::new(format!(
                "speed: {:.0}x\nstep delta: {}ms\nbehind: {}ms",
                self.speed,
                self.time_per_step.as_millis(),
                self.time_to_simulate.as_millis(),
            )),
            DrawParam::default().dest([width - 200.0, 0.0]),
        );

        canvas.finish(ctx)
    }
}
