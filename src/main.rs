#![allow(dead_code)]

pub mod application_context;
mod configurations;
mod environment;
mod environment_awareness;
mod layout_info;
mod organisms;
pub mod simulation;
pub mod simulation_thread;
pub mod vector_helper;

use std::time::Duration;
use std::{env, fs};

use application_context::ApplicationContext;
use configurations::generation_configuration::GenerationConfiguration;
use configurations::species_generation_configuration::SpeciesGenerationConfiguration;
use environment::Environment;
use ggez::conf::WindowSetup;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, BlendMode, Color, DrawParam, Text};

use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};
use organisms::species::{HuntingBehavior, Nutrition, Species};

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
    species_gen_config: GenerationConfiguration,
    application_context: ApplicationContext,
    speed: u32,
}

impl MyGame {
    pub fn new(ctx: &mut Context) -> MyGame {
        let species_gen_config = Self::get_generation_config();
        let time_step = Duration::from_secs_f32(0.05);
        let environment = Environment::new(ctx, time_step, &species_gen_config);
        MyGame {
            species_gen_config,
            time_to_simulate: Duration::ZERO,
            environment,
            time_per_step: time_step,
            application_context: ApplicationContext::default(),
            speed: 1,
        }
    }

    fn print_env_generation_config(&self) {
        let serialization = serde_json::to_string(&self.species_gen_config);

        if let Ok(json) = serialization {
            println!("{}", json);
        } else {
            println!("Serialization failed.");
        }
    }

    fn load_env_generation_config() -> Option<GenerationConfiguration> {
        if let Some(curr_dir) = env::current_dir().ok()?.to_str() {
            let assets_dir = curr_dir.to_owned() + "/assets";
            let config_file = assets_dir + "/default_species_config.json";
            let json = fs::read_to_string(config_file).ok()?;
            let config = serde_json::from_str(json.as_str()).ok()?;
            return config;
        }
        None
    }

    fn get_generation_config() -> GenerationConfiguration {
        if let Some(config) = Self::load_env_generation_config() {
            config
        } else {
            println!("Loading species config json failed.");
            generate_default_generation_configuration()
        }
    }

    fn restart(&mut self, ctx: &mut Context) {
        let species_gen_config = Self::get_generation_config();
        self.species_gen_config = species_gen_config.to_owned();

        self.time_to_simulate = Duration::ZERO;

        self.environment = Environment::new(ctx, self.time_per_step, &species_gen_config);
    }
}

fn generate_default_generation_configuration() -> GenerationConfiguration {
    GenerationConfiguration {
        species: vec![
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Herbivore"),
                    max_energy: 256.0,
                    max_health: 40.0,
                    max_age: Duration::from_secs(60),
                    energy_cost_of_birth: 10.0,
                    walk_speed_s: 2.8,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(0, 91, 150),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Plant,
                    eyesight_distance: 25.0,
                    birth_distance: 1.3,
                    birth_immunity: Duration::ZERO,
                    health_cost_of_birth: 20.0,
                    eating_distance: 0.2,
                    max_per_meter: 0.0,
                    hunting_behavior: HuntingBehavior::Random,
                },
                amount_per_meter: 0.2,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Plant"),
                    max_energy: 150.0,
                    max_health: 30.0,
                    health_cost_of_birth: 30.0,
                    max_age: Duration::from_secs(30),
                    energy_cost_of_birth: 10.0,
                    walk_speed_s: 0.0,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(10, 70, 10),
                    contained_nutrition: Nutrition::Plant,
                    eats: Nutrition::Corpse,
                    eyesight_distance: 40.0,
                    birth_distance: 40.0,
                    birth_immunity: Duration::from_secs(5),
                    eating_distance: 55.0,
                    // should increase eating distance to eat corpses
                    max_per_meter: 2.0,
                    hunting_behavior: HuntingBehavior::Random,
                },
                amount_per_meter: 0.6,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Carnivore"),
                    max_energy: 120.0,
                    max_health: 70.0,
                    health_cost_of_birth: 20.0,
                    max_age: Duration::from_secs(70),
                    energy_cost_of_birth: 20.0,
                    walk_speed_s: 12.9,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(200, 0, 0),
                    contained_nutrition: Nutrition::Meat,
                    eats: Nutrition::Meat,
                    eyesight_distance: 18.0,
                    birth_distance: 0.1,
                    birth_immunity: Duration::ZERO,
                    eating_distance: 0.2,
                    max_per_meter: 0.0,
                    hunting_behavior: HuntingBehavior::Random,
                },
                amount_per_meter: 0.04,
            },
            SpeciesGenerationConfiguration {
                species: Species {
                    name: String::from("Scavenger"),
                    max_energy: 150.0,
                    max_health: 30.0,
                    health_cost_of_birth: 1.0,
                    max_age: Duration::from_secs(250),
                    energy_cost_of_birth: 60.0,
                    walk_speed_s: 18.2,
                    photosynthesis_rate_s: 0.0,
                    color: Color::from_rgb(100, 0, 150),
                    contained_nutrition: Nutrition::None,
                    eats: Nutrition::Corpse,
                    eyesight_distance: 250.0,
                    birth_distance: 4.7,
                    birth_immunity: Duration::ZERO,
                    eating_distance: 0.2,
                    max_per_meter: 0.0,
                    hunting_behavior: HuntingBehavior::Random,
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
        let time_per_step_step = Duration::from_secs_f32(0.1);
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
            Some(VirtualKeyCode::Period) => {
                self.time_per_step += time_per_step_step;
                self.environment.change_time_step(self.time_per_step);
            }
            Some(VirtualKeyCode::Comma) => {
                if let Some(new_value) = self.time_per_step.checked_sub(time_per_step_step) {
                    self.time_per_step = new_value.max(time_per_step_step);
                }
                self.environment.change_time_step(self.time_per_step);
            }
            Some(VirtualKeyCode::E) => self.print_env_generation_config(),
            Some(VirtualKeyCode::R) => self.restart(_ctx),
            _ => self.environment.key_down_event(_ctx, input, _repeated),
        };
        Ok(())
    }

    fn key_up_event(
        &mut self,
        ctx: &mut Context,
        input: ggez::input::keyboard::KeyInput,
    ) -> GameResult {
        self.environment.key_up_event(ctx, input);
        Ok(())
    }

    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.time_to_simulate += ctx.time.delta() * self.speed;

        let steps_to_simulate: u32 =
            (self.time_to_simulate.as_millis() / self.time_per_step.as_millis()) as u32;
        self.time_to_simulate -= self.time_per_step * steps_to_simulate;
        self.environment.simulate(steps_to_simulate);

        self.speed = self.speed.clamp(0, 32);

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        self.environment.handle_camera_controls(ctx);

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
