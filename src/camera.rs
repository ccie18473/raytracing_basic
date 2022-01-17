use crate::prelude::*;

const LEFT_KEY: KeyCode = ggez::event::KeyCode::Left;
const RIGHT_KEY: KeyCode = ggez::event::KeyCode::Right;
const UP_KEY: KeyCode = ggez::event::KeyCode::Up;
const DOWN_KEY: KeyCode = ggez::event::KeyCode::Down;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Action {
    None = 0,
    LookLeft,
    LookRight,
    MoveForward,
    MoveBackward,
}

// viewport_size = 1 x 1
// projection_plane_d = 1
// gives a FOV of 53 degrees
pub struct Camera {
    pub pos: Vector3<f64>,
    pub dir: Vector3<f64>,
    pub angle: f64,
    pub viewport_w: f64,
    pub viewport_h: f64,
    pub projection_plane_d: f64,
    pub action: Action,
}

impl Camera {
    pub fn new() -> Camera {
        let pos = Vector3::new(0.0, 0.0, 0.0);
        let dir = Vector3::new(0.0, 0.0, 1.0);
        Camera {
            pos,
            dir,
            angle: 0.0,
            viewport_w: 1.0,
            viewport_h: 1.0,
            projection_plane_d: 1.0,
            action: Action::None,
        }
    }
    //
    // convert from canvas coordinates 0..+x, 0..+y
    // to viewport coordinates -x..+x, -y..+y
    //
    pub fn canvas_to_viewport(
        &mut self,
        x: f64,
        y: f64,
        canvas_w: f64,
        canvas_h: f64,
    ) -> Vector3<f64> {
        let mut viewport_point = Vector3::new(0.0, 0.0, 0.0);

        viewport_point.x = (x - canvas_w / 2.0) * self.viewport_w / canvas_w;
        viewport_point.y = (canvas_h / 2.0 - y) * self.viewport_h / canvas_h;
        viewport_point.z = self.projection_plane_d;

        viewport_point
    }
    //
    // rotate xz-plane coordinates
    // around y-axis
    //
    pub fn rotate_viewport(&mut self, point: &mut Vector3<f64>) {
        let old_point_x = point.x;
        let old_point_z = point.z;
        point.x = old_point_x * (self.angle).cos() - old_point_z * (self.angle).sin();
        point.z = old_point_x * (self.angle).sin() + old_point_z * (self.angle).cos();
    }
    pub fn handle_inputs(&mut self, keycode: KeyCode, pressed: bool) {
        if keycode == UP_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::MoveForward;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == DOWN_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::MoveBackward;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == LEFT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::LookLeft;
                }
            } else {
                self.action = Action::None;
            }
        } else if keycode == RIGHT_KEY {
            if pressed {
                if self.action == Action::None {
                    self.action = Action::LookRight;
                }
            } else {
                self.action = Action::None;
            }
        }
    }

    pub fn update(&mut self, delta: f64) {
        //
        // speed modifiers
        //
        let move_speed = 2.0 * delta;
        let rot_speed = 1.0 * delta;
        //
        // move on the xz-plane
        //
        if self.action == Action::MoveForward {
            self.pos.x += self.dir.x * move_speed;
            self.pos.z += self.dir.z * move_speed;
        } else if self.action == Action::MoveBackward {
            self.pos.x -= self.dir.x * move_speed;
            self.pos.z -= self.dir.z * move_speed;
        //
        // rotate around the y-axis
        //
        } else if self.action == Action::LookLeft {
            //
            // both camera direction and viewport must be rotated
            //
            let old_dir_x = self.dir.x;
            self.dir.x = self.dir.x * (rot_speed).cos() - self.dir.z * (rot_speed).sin();
            self.dir.z = old_dir_x * (rot_speed).sin() + self.dir.z * (rot_speed).cos();
            //
            // angle for viewport rotation
            //
            self.angle += rot_speed;
        } else if self.action == Action::LookRight {
            //
            // both camera direction and viewport must be rotated
            //
            let old_dir_x = self.dir.x;
            self.dir.x = self.dir.x * (-rot_speed).cos() - self.dir.z * (-rot_speed).sin();
            self.dir.z = old_dir_x * (-rot_speed).sin() + self.dir.z * (-rot_speed).cos();
            //
            // angle for viewport rotation
            //
            self.angle -= rot_speed;
        } else {
            self.action = Action::None;
        }
    }
}
