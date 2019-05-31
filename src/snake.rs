use piston::input::{RenderArgs};
use opengl_graphics::{GlGraphics};
use graphics::{rectangle, Transformed};

#[derive(Clone)]
struct Box {
	x: i32,
	y: i32,
}

impl Box {
	fn hit_screen(&mut self, args: RenderArgs) -> bool{
		self.x < 0 || self.x > args.width as i32 ||
			self.y < 0 || self.y > args.height as i32
	}
}

pub struct Snake {
	head: Box,
	body: Vec<Box>,
	vel: Vec<i32>,
	size: i32,
	speed: i32
}

impl Snake {

	pub fn new(x: i32, y: i32, size: i32, speed: i32) -> Snake{
		Snake{
			head: Box{x, y},
			body: vec![Box{x:-20, y:0}, Box{x:-40, y:0}],
			vel: vec![0,0],
			size: size,
			speed: speed
		}
	}

	pub fn render(&mut self, gl: &mut GlGraphics, args: &RenderArgs){
		self.head.hit_screen(*args);
		const COLOR: [f32; 4] = [1.0, 0.5, 0.5, 1.0];
		let rect = rectangle::square(0.0, 0.0, self.size as f64);
		gl.draw(args.viewport(), |c, gl| {
			rectangle(COLOR, rect, c.transform.trans(self.head.x as f64, self.head.y as f64), gl);
			for body in self.body.iter(){
				rectangle(COLOR, rect, c.transform.trans(body.x as f64, body.y as f64), gl);
			}
		});
		
	}

	pub fn update(&mut self) {

		self.body.insert(0, self.head.clone());
		self.head.x += self.vel[0] * self.size;
		self.head.y += self.vel[1] * self.size;

		self.body.pop();
	}

	pub fn up(&mut self) {
		self.vel[0] = 0;
		self.vel[1] = -self.speed;
	}

	pub fn down(&mut self){
		self.vel[0] = 0;
		self.vel[1] = self.speed;
	}

	pub fn left(&mut self){
		self.vel[0] = -self.speed;
		self.vel[1] = 0;
	}

	pub fn right(&mut self){
		self.vel[0] = self.speed;
		self.vel[1] = 0;
	}

}