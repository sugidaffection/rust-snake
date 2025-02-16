use piston_window::graphics::Graphics;
use piston_window::{clear, math, prelude::*};

use crate::food::Food;
use crate::snake::Snake;

pub struct App {
    snake: Snake,
    food: Food,
    height: f64,
    width: f64,
}

impl App {
    pub fn new(grid_size: f64, window_size: Size) -> Self {
        let mut food = Food::new(grid_size);
        food.spawn(window_size.width, window_size.height);

        Self {
            snake: Snake::new(grid_size),
            food,
            height: window_size.height,
            width: window_size.width,
        }
    }

    pub fn render<B: Graphics>(&mut self, t: math::Matrix2d, b: &mut B) {
        const BACKGROUND: [f32; 4] = [0.15, 0.15, 0.15, 1.0];

        clear(BACKGROUND, b);

        self.food.render(t, b);
        self.snake.render(t, b);
    }

    pub fn render_update(&mut self, args: &RenderArgs) {
        // Update window size
        self.width = args.window_size[0];
        self.height = args.window_size[1];
    }

    pub fn update(&mut self, args: &UpdateArgs) {
        self.snake.update(args.dt);

        if self.snake.eat(&self.food) {
            self.food.spawn(self.width, self.height);
        }
    }

    pub fn handle_input(&mut self, key: Key) {
        match key {
            Key::Up => self.snake.up(),
            Key::Down => self.snake.down(),
            Key::Left => self.snake.left(),
            Key::Right => self.snake.right(),
            _ => {}
        }
    }
}
