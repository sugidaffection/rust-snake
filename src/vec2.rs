use rand::random_range;

#[derive(Clone, Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }

    pub fn collide(&self, target: &Vec2) -> bool {
        self.x == target.x && self.y == target.y
    }

    pub fn generate_pos(max: f64, step: f64) -> f64 {
        let n = random_range(0..(max / step) as i32) as f64 * step;
        n
    }

    pub fn random_pos(&mut self, max_x: f64, max_y: f64, step: f64) {
        self.x = Self::generate_pos(max_x, step);
        self.y = Self::generate_pos(max_y, step);
    }
}
