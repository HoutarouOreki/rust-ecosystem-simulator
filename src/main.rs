#![allow(dead_code)]

pub mod application_context;
mod configurations;
mod environment;
mod layout_info;
mod organisms;
pub mod vector_helper;

use std::time::Duration;

use application_context::ApplicationContext;
use configurations::generation_configuration::GenerationConfiguration;
use configurations::species_generation_configuration::SpeciesGenerationConfiguration;
use environment::Environment;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, BlendMode, Color};

use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use organisms::species::{Nutrition, Species};

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

struct MyGame {
    time_since_last_simulation_step: Duration,
    time_per_step: Duration,
    environment: Environment,
    application_context: ApplicationContext,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let generation_configuration = generate_default_generation_configuration();
        MyGame {
            time_since_last_simulation_step: Duration::ZERO,
            environment: Environment::new(&generation_configuration),
            time_per_step: Duration::from_secs_f32(0.05),
            application_context: ApplicationContext::default(),
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
                    max_age: Duration::from_secs(200),
                    cost_of_birth: 30.0,
                    walk_speed_s: 0.2,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(0, 91, 150),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Plant,
                },
                amount_per_meter: 0.1,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Plant"),
                    max_energy: 80.0,
                    max_health: 30.0,
                    max_age: Duration::from_secs(19),
                    cost_of_birth: 10.0,
                    walk_speed_s: 0.0,
                    photosynthesis_rate_s: 3.0,
                    color: Color::from_rgb(79, 121, 66),
                    contained_nutrition: Nutrition::Plant,
                    eats: Nutrition::None,
                },
                amount_per_meter: 0.16,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Carnivore"),
                    max_energy: 30.0,
                    max_health: 30.0,
                    max_age: Duration::from_secs(64),
                    cost_of_birth: 12.0,
                    walk_speed_s: 2.8,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(200, 0, 0),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Meat,
                },
                amount_per_meter: 0.01,
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

        self.environment.handle_camera_controls(ctx);

        if ctx.keyboard.is_key_just_pressed(VirtualKeyCode::H) {
            self.application_context.draw_each_organism_info =
                !self.application_context.draw_each_organism_info;
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        canvas.set_blend_mode(BlendMode::REPLACE);
        canvas.set_premultiplied_text(false);

        self.environment
            .draw(&mut canvas, ctx, &self.application_context);

        canvas.finish(ctx)
    }
}
