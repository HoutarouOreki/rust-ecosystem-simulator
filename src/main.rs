#![allow(dead_code)]

mod environment;
mod layout_info;
mod organisms;

use std::collections::hash_map::RandomState;
use std::collections::HashMap;

use environment::Environment;
use ggez::event::{self, EventHandler};
use ggez::graphics::{self, Color};

use ggez::input::keyboard::KeyboardContext;

use ggez::winit::event::VirtualKeyCode;
use ggez::{Context, ContextBuilder, GameResult};

fn main() {
    // Make a Context.
    let (mut ctx, event_loop) = ContextBuilder::new("my_game", "HoutarouOreki")
        .build()
        .expect("aieee, could not create ggez context!");

    // Create an instance of your event handler.
    // Usually, you should provide it with the Context object to
    // use when setting your game up.
    let my_game = MyGame::new(&mut ctx);
    ctx.gfx.set_window_title("HHHHHhhhhhhhhhhhhhhhhhhhh");
    let _resize_result = ctx.gfx.set_resizable(true);

    // Run!
    event::run(ctx, event_loop, my_game);
}

const CAMERA_SPEED: f32 = 200.0;

struct MyGame {
    seconds_since_last_simulation_step: f32,
    seconds_per_step: f32,
    environment: Environment,
    key_dictionary: HashMap<VirtualKeyCode, [f32; 2], RandomState>,
}

impl MyGame {
    pub fn new(_ctx: &mut Context) -> MyGame {
        MyGame {
            seconds_since_last_simulation_step: 0.0,
            environment: Environment::new(),
            seconds_per_step: 0.01,
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

        self.environment.offset = vecmath::vec2_add(
            self.environment.offset.into(),
            vecmath::vec2_scale(
                vecmath::vec2_normalized(camera_moving_direction),
                CAMERA_SPEED * ctx.time.delta().as_secs_f32(),
            ),
        )
        .into();
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

impl EventHandler for MyGame {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        self.seconds_since_last_simulation_step += ctx.time.delta().as_secs_f32();

        if self.seconds_since_last_simulation_step > self.seconds_per_step {
            self.seconds_since_last_simulation_step -= self.seconds_per_step;
            self.environment.simulate(self.seconds_per_step);
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
