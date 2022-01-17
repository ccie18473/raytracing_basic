extern crate glam;
extern crate good_web_game as ggez;

pub const SCREEN_WIDTH: i32 = 640;
pub const SCREEN_HEIGHT: i32 = 480;

mod camera;
mod canvas;
mod game;
mod ray;

mod prelude {
    pub use crate::camera::*;
    pub use crate::canvas::*;
    pub use crate::game::*;
    pub use crate::ray::*;
    pub use cgmath::{InnerSpace, Point2, Vector3};
    pub use ggez::event::{EventHandler, KeyCode, KeyMods, MouseButton};
    pub use ggez::graphics::{Color, Image, Rect};
    pub use ggez::timer;
    pub use ggez::{Context, GameResult};
    pub use std::path;
}

use prelude::*;

struct MainState {
    game: Game,
}

impl MainState {
    fn new(ctx: &mut Context) -> GameResult<MainState> {
        let (w, h) = ggez::graphics::drawable_size(ctx);

        let game = Game::new(ctx, w as isize, h as isize);

        let s = MainState { game };

        Ok(s)
    }
}
impl ggez::event::EventHandler<ggez::GameError> for MainState {
    fn update(&mut self, _ctx: &mut Context) -> ggez::GameResult {
        const DESIRED_FPS: u32 = 60;

        let delta = 1.0 / DESIRED_FPS as f64;

        self.game.update(delta);

        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> ggez::GameResult {
        ggez::graphics::clear(ctx, Color::BLACK);

        self.game.canvas.draw(ctx, &mut self.game.camera)?;

        self.game.canvas.draw_fps(ctx)?;

        ggez::graphics::present(ctx)?;

        Ok(())
    }
    fn key_down_event(
        &mut self,
        _ctx: &mut Context,
        keycode: KeyCode,
        _keymod: KeyMods,
        _repeat: bool,
    ) {
        self.game.camera.handle_inputs(keycode, true);
    }

    fn key_up_event(&mut self, _ctx: &mut Context, keycode: KeyCode, _keymod: KeyMods) {
        self.game.camera.handle_inputs(keycode, false);
    }
    fn resize_event(&mut self, ctx: &mut Context, w: f32, h: f32) {
        self.game.new_size(ctx, w as isize, h as isize);

        let coordinates = ggez::graphics::Rect::new(0.0, 0.0, w, h);

        ggez::graphics::set_screen_coordinates(ctx, coordinates).expect("Can't resize the window");
    }
}
pub fn main() -> GameResult {
    let conf = ggez::conf::Conf::default()
        .window_width(SCREEN_WIDTH)
        .window_height(SCREEN_HEIGHT)
        .window_resizable(true)
        .window_title("raytracing_basic v1.0.0, 2022".to_string());

    ggez::start(conf, |mut context| {
        Box::new(MainState::new(&mut context).unwrap())
    })
}
