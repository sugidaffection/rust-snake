extern crate piston;
extern crate graphics;
extern crate opengl_graphics;
extern crate glutin_window;

use piston::window::WindowSettings;
use piston::event_loop::{EventSettings, Events, EventLoop};
use piston::input::{Button, Key, PressEvent, RenderArgs, RenderEvent, UpdateArgs, UpdateEvent};
use glutin_window::GlutinWindow;
use opengl_graphics::{GlGraphics, OpenGL};

mod snake;
use snake::Snake;

pub struct App {
	gl: GlGraphics,
	snake: Snake
}

impl App {
	fn render(&mut self, args: &RenderArgs){
		use graphics::clear;
		
		const BACKGROUND: [f32; 4] = [0.0, 0.0, 0.0, 1.0];

		self.gl.draw(args.viewport(), |_, gl| {
			clear(BACKGROUND, gl);
		});

		self.snake.render(&mut self.gl, args);
	}

	fn update(&mut self, args: &UpdateArgs){
		self.snake.update();
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

	let opengl = OpenGL::V4_1;
	let mut window: GlutinWindow = WindowSettings::new("Snake", [400, 400])
		.opengl(opengl)
		.exit_on_esc(true)
		.build()
		.unwrap();
	
	let mut app = App {
		gl: GlGraphics::new(opengl),
		snake: Snake::new(0, 0, 20, 1),
	};

	let mut events = Events::new(EventSettings::new()).ups(15);

	while let Some(e) = events.next(&mut window){
		if let Some(r) = e.render_args() {
			app.render(&r);
		}

		if let Some(u) = e.update_args() {
			app.update(&u);
		}

		if let Some(b) = e.press_args() {
			app.press(&b);
		}
	}

}