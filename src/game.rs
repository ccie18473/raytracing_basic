use crate::prelude::*;

pub struct Game {
    pub camera: Camera,
    pub canvas: Canvas,
}

impl Game {
    pub fn new(ctx: &mut Context, width: isize, height: isize) -> Game {
        Game {
            camera: Camera::new(),
            canvas: Canvas::new(ctx, width, height),
        }
    }
    pub fn new_size(&mut self, ctx: &mut Context, width: isize, height: isize) {
        self.canvas = Canvas::new(ctx, width, height);
    }
    pub fn update(&mut self, delta: f64) {
        self.camera.update(delta);
    }
}
