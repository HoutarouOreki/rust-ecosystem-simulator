#![allow(dead_code)]

mod configurations;
mod environment;
mod layout_info;
mod organisms;

use std::time::Duration;

use configurations::generation_configuration::GenerationConfiguration;
use configurations::species_generation_configuration::SpeciesGenerationConfiguration;
use environment::Environment;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};

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

struct MyGame {
    time_since_last_simulation_step: Duration,
    time_per_step: Duration,
    environment: Environment,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        let generation_configuration = generate_default_generation_configuration();
        MyGame {
            time_since_last_simulation_step: Duration::ZERO,
            environment: Environment::new(&generation_configuration),
            time_per_step: Duration::from_secs_f32(0.05),
        }
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
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Short-lived species"),
                    max_energy: 30,
                    max_health: 30,
                    max_age: Duration::from_secs(5),
                    cost_of_birth: 20,
                    can_walk: false,
                    can_eat_organisms: false,
                    can_photosynthesize: false,
                    color: Color::from_rgb(200, 0, 0),
                },
                amount_per_meter: 0.1,
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

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        let mut canvas = graphics::Canvas::from_frame(ctx, Color::BLACK);

        self.environment.draw(&mut canvas, ctx);

        canvas.finish(ctx)
    }
}
