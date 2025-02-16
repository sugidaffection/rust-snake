use piston_window::{math::Matrix2d, rectangle, Graphics, Transformed};

use crate::vec2::Vec2;

pub struct Food {
    pub pos: Vec2,
    pub size: f64,
}

impl Food {
    pub fn new(size: f64) -> Self {
        Self {
            pos: Vec2::new(0.0, 0.0),
            size,
        }
    }

    pub fn spawn(&mut self, w: f64, h: f64) {
        self.pos.random_pos(w, h, self.size);
    }

    pub fn render<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        const COLOR: [f32; 4] = [1.0, 0.2, 0.2, 1.0];
        let food_size = self.size as f64 * 0.6;
        let offset = (self.size as f64 - food_size) / 2.0;

        rectangle(
            COLOR,
            [offset, offset, food_size, food_size],
            t.trans(self.pos.x as f64, self.pos.y as f64),
            g,
        );
    }
}
