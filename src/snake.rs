use crate::math::Matrix2d;
use graphics::{ellipse, rectangle, DrawState, Graphics, Transformed};
use rand::{thread_rng, Rng};

#[derive(Clone)]
struct Vec2 {
    x: i32,
    y: i32,
}

impl Vec2 {
    fn collide(&mut self, target: &Vec2) -> bool {
        self.x == target.x && self.y == target.y
    }

    fn generate_pos(&mut self, max: i32, step: i32) -> i32 {
        let mut thread = thread_rng();
        let n = thread.gen_range(0..max);
        if n % step == 0 {
            return n;
        } else {
            return self.generate_pos(max, step);
        }
    }

    fn random_pos(&mut self, max_x: i32, max_y: i32, step: i32) {
        self.x = self.generate_pos(max_x, step);
        self.y = self.generate_pos(max_y, step);
    }
}

pub struct Food {
    pos: Vec2,
    size: i32,
}

impl Food {
    pub fn new(size: i32) -> Food {
        let food = Food {
            pos: Vec2 { x: 0, y: 0 },
            size: size,
        };

        food
    }

    /// Spawn food in random position inside screen
    pub fn spawn(&mut self, w: i32, h: i32) {
        self.pos.random_pos(w, h, self.size);
    }

    pub fn render<B: Graphics>(&mut self, t: Matrix2d, b: &mut B) {
        const COLOR: [f32; 4] = [1.0, 0.0, 0.0, 1.0];
        ellipse::Ellipse::new(COLOR).draw(
            [0.0, 0.0, self.size as f64, self.size as f64],
            &DrawState::default(),
            t.trans(self.pos.x as f64, self.pos.y as f64),
            b,
        );
    }
}

pub struct Snake {
    head: Vec2,
    body: Vec<Vec2>,
    vel: Vec<i32>,
    size: i32,
    speed: i32,
    dead: bool,
}

impl Snake {
    pub fn new(x: i32, y: i32, size: i32, speed: i32) -> Snake {
        Snake {
            head: Vec2 { x, y },
            body: vec![],
            vel: vec![0, 0],
            size: size,
            speed: speed,
            dead: false,
        }
    }

    // Check if snake is hit screen border
    pub fn hit_screen(&mut self, width: i32, height: i32) -> bool {
        self.head.x < 0 || self.head.x > width || self.head.y < 0 || self.head.y > height
    }

    /// Check if snake is colliding with bodies
    pub fn hit_body(&mut self) -> bool {
        for (i, body) in self.body.iter().enumerate() {
            if i > 0 && self.head.collide(&body) {
                return true;
            }
        }

        return false;
    }

    pub fn render<B: Graphics>(&mut self, t: Matrix2d, b: &mut B) {
        const HEAD: [f32; 4] = [1.0, 0.8, 0.5, 1.0];
        const BODY: [f32; 4] = [1.0, 0.5, 0.8, 1.0];
        let rect = rectangle::square(0.0, 0.0, self.size as f64);
        for body in self.body.iter() {
            rectangle(BODY, rect, t.trans(body.x as f64, body.y as f64), b);
        }
        rectangle(
            HEAD,
            rect,
            t.trans(self.head.x as f64, self.head.y as f64),
            b,
        );
    }

    pub fn eat(&mut self, food: &Food) -> bool {
        let pos = &food.pos;
        // check if snake head is colliding with food
        if self.head.collide(pos) {
            // create new tail from head position if body is empty
            // else create from current tail position
            self.body.push(if self.body.len() > 0 {
                self.body.last().unwrap().clone()
            } else {
                self.head.clone()
            });

            return true;
        }

        false
    }

    pub fn update(&mut self) {
        // check if snake alive
        if !self.dead {
            // insert new body from current head position
            self.body.insert(0, self.head.clone());
            // update head position
            self.head.x += self.vel[0] * self.size;
            self.head.y += self.vel[1] * self.size;
            // remove tail
            self.body.pop();
        }
    }

    pub fn up(&mut self) {
        if self.vel[1] == 0 {
            self.vel[0] = 0;
            self.vel[1] = -self.speed;
        }
    }

    pub fn down(&mut self) {
        if self.vel[1] == 0 {
            self.vel[0] = 0;
            self.vel[1] = self.speed;
        }
    }

    pub fn left(&mut self) {
        if self.vel[0] == 0 {
            self.vel[0] = -self.speed;
            self.vel[1] = 0;
        }
    }

    pub fn right(&mut self) {
        if self.vel[0] == 0 {
            self.vel[0] = self.speed;
            self.vel[1] = 0;
        }
    }
}
