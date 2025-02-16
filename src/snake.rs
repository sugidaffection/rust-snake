use piston_window::{rectangle, Graphics, Transformed};

use crate::{food::Food, math::Matrix2d, vec2::Vec2};

#[derive(PartialEq, Debug)]
pub enum DirectionState {
    LEFT,
    RIGHT,
    UP,
    DOWN,
}

#[derive(Debug)]
pub struct Snake {
    head: Vec2,
    body: Vec<Vec2>,
    size: f64,
    start_position: Vec2,
    target_position: Vec2,
    position: Vec2,
    dir_state: DirectionState,
    dir_move: Vec2,
    time_elapsed: f64,
    animation_delay: f64,
    movement_speed: f64,
}

impl Snake {
    pub fn new(size: f64) -> Self {
        Self {
            head: Vec2::new(0.0, 0.0),
            body: vec![],
            size,
            start_position: Vec2::new(0.0, 0.0),
            target_position: Vec2::new(20.0, 0.0),
            position: Vec2::new(0.0, 0.0),
            dir_state: DirectionState::RIGHT,
            dir_move: Vec2::new(0.0, 0.0),
            time_elapsed: 0.0,
            animation_delay: 0.4,
            movement_speed: 0.6,
        }
    }

    pub fn render<G: Graphics>(&self, t: Matrix2d, g: &mut G) {
        const HEAD_COLOR: [f32; 4] = [0.0, 1.0, 0.6, 1.0];
        const BODY_COLOR: [f32; 4] = [0.3, 0.3, 0.3, 1.0];
        let rect = rectangle::square(0.0, 0.0, self.size as f64);

        for segment in &self.body {
            rectangle(
                BODY_COLOR,
                rect,
                t.trans(segment.x as f64, segment.y as f64),
                g,
            );
        }

        rectangle(
            HEAD_COLOR,
            rect,
            t.trans(self.head.x as f64, self.head.y as f64),
            g,
        );
    }

    pub fn eat(&mut self, food: &Food) -> bool {
        if self.head.collide(&food.pos) {
            let last_segment = self.body.last().unwrap_or(&self.head);
            self.body.push(last_segment.clone());

            self.movement_speed *= 0.9;

            true
        } else {
            false
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.movement_speed = self.movement_speed.max(0.1);
        self.animation_delay = self.animation_delay.max(0.1).min(self.movement_speed);
        self.time_elapsed += dt;

        if self.time_elapsed >= self.movement_speed {
            self.time_elapsed = 0.0; // Reset timer

            // Update direction
            match self.dir_state {
                DirectionState::LEFT => {
                    self.dir_move.x = -1.0;
                    self.dir_move.y = 0.0;
                }
                DirectionState::RIGHT => {
                    self.dir_move.x = 1.0;
                    self.dir_move.y = 0.0;
                }
                DirectionState::UP => {
                    self.dir_move.y = -1.0;
                    self.dir_move.x = 0.0;
                }
                DirectionState::DOWN => {
                    self.dir_move.y = 1.0;
                    self.dir_move.x = 0.0;
                }
            }

            // Set new start and target positions
            self.start_position.x = (self.position.x / self.size).round() * self.size;
            self.start_position.y = (self.position.y / self.size).round() * self.size;

            let last_target = self.target_position.clone();

            self.target_position.x = self.start_position.x + (self.dir_move.x * self.size);
            self.target_position.y = self.start_position.y + (self.dir_move.y * self.size);

            if !self.body.is_empty() {
                for i in (1..self.body.len()).rev() {
                    self.body[i] = self.body[i - 1].clone();
                }
                self.body[0] = last_target; // First segment moves to last head position
            }
        }

        // Calculate progress (normalized 0.0 to 1.0)
        let t = (self.time_elapsed / self.animation_delay).clamp(0.0, 1.0);

        // Apply ease-in function: Quadratic acceleration
        let eased_t = t * t;

        // Interpolate smoothly
        self.position.x =
            self.start_position.x + (self.target_position.x - self.start_position.x) * eased_t;
        self.position.y =
            self.start_position.y + (self.target_position.y - self.start_position.y) * eased_t;

        // Update snake head and body
        self.head.x = self.position.x;
        self.head.y = self.position.y;
    }

    pub fn up(&mut self) {
        if self.dir_state != DirectionState::DOWN {
            self.dir_state = DirectionState::UP;
        }
    }

    pub fn down(&mut self) {
        if self.dir_state != DirectionState::UP {
            self.dir_state = DirectionState::DOWN;
        }
    }

    pub fn left(&mut self) {
        if self.dir_state != DirectionState::RIGHT {
            self.dir_state = DirectionState::LEFT;
        }
    }

    pub fn right(&mut self) {
        if self.dir_state != DirectionState::LEFT {
            self.dir_state = DirectionState::RIGHT;
        }
    }
}
