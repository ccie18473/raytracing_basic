use crate::prelude::*;

type Point2 = glam::Vec2;

pub struct Canvas {
    pub width: isize,
    pub height: isize,
}

impl Canvas {
    pub fn new(_ctx: &mut Context, width: isize, height: isize) -> Canvas {
        Canvas { width, height }
    }
    pub fn draw(&mut self, ctx: &mut Context, camera: &mut Camera) -> GameResult {
        let rays = crate::Rays::new(camera, self.width, self.height);

        let p = ggez::graphics::DrawParam::new().dest(Point2::new(0.0, 0.0));
        let image = ggez::graphics::Image::from_rgba8(
            ctx,
            self.width as u16,
            self.height as u16,
            &rays.buffer,
        )?;

        ggez::graphics::draw(ctx, &image, p)?;

        Ok(())
    }
    pub fn draw_fps(&mut self, ctx: &mut Context) -> GameResult {
        let fps = ggez::timer::fps(ctx);
        let fps_display = ggez::graphics::Text::new(format!("FPS: {:.2}", fps));
        let dest = Point2::new(0.0, 0.0);
        let params = ggez::graphics::DrawParam::default()
            .dest(dest)
            .color(Color::WHITE);
        ggez::graphics::draw(ctx, &fps_display, params)?;

        Ok(())
    }
}
