use piston_window::*;
mod snake;
use snake::{Food, Snake};

pub struct App {
    snake: Snake,
    food: Food,
    height: i32,
    width: i32,
}

impl App {
    /// Reset game state
    fn reset(&mut self) {
        self.snake = Snake::new(0, 0, 20, 1);
        let mut food = Food::new(20);
        food.spawn(400, 400);
        self.food = food;
    }

    fn render<B: Graphics>(&mut self, t: math::Matrix2d, b: &mut B) {
        const BACKGROUND: [f32; 4] = [0.3, 0.3, 0.3, 1.0];

        clear(BACKGROUND, b);

        self.food.render(t, b);
        self.snake.render(t, b);
    }

    fn render_update(&mut self, args: &RenderArgs) {
        // Update window size
        self.width = args.window_size[0] as i32;
        self.height = args.window_size[1] as i32;
    }

    fn update(&mut self, _args: &UpdateArgs) {
        self.snake.update();
        let food = &self.food;
        if self.snake.eat(food) {
            self.food.spawn(self.width as i32, self.height as i32);
        }

        if self.snake.hit_screen(self.width, self.height) || self.snake.hit_body() {
            self.reset();
        }
    }

    fn press(&mut self, args: &Button) {
        if let &Button::Keyboard(key) = args {
            match key {
                Key::Up => {
                    self.snake.up();
                }
                Key::Down => {
                    self.snake.down();
                }
                Key::Left => {
                    self.snake.left();
                }
                Key::Right => {
                    self.snake.right();
                }
                _ => {}
            }
        }
    }
}

fn main() {
    let mut window: PistonWindow = WindowSettings::new("Snake", (640, 480))
        .exit_on_esc(true)
        .build()
        .unwrap_or_else(|e| panic!("Failed to build PistonWindow: {}", e));

    // update per seconds
    window.set_ups(15);

    let mut food = Food::new(20);
    food.spawn(400, 400);

    let mut app = App {
        snake: Snake::new(0, 0, 20, 1),
        food: food,
        height: 400,
        width: 400,
    };

    while let Some(e) = window.next() {
        window.draw_2d(&e, |c, g, _d| {
            app.render(c.transform, g);
        });
        if let Some(r) = e.render_args() {
            app.render_update(&r);
        }

        if let Some(u) = e.update_args() {
            app.update(&u);
        }

        if let Some(b) = e.press_args() {
            app.press(&b);
        }
    }
}
