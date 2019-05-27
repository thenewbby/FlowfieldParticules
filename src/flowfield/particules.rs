extern crate rand;
// mod flowfield;
use rand::Rng;

use euclid::*;
use super::{SQUARE_SIZE, WIDTH, HEIGHT};

pub const MAX_SPEED: f64 = 5.0;
pub const ACCEL: f64 = 0.2;

#[derive(Debug)]
pub struct Particule {
	pub _pos: Point2D<i32>,
	pub _pos_last: Point2D<i32>,
	_speed: Vector2D<f64>,
	// _acc: Vector2D<u32>,
}


impl Particule {

	pub fn new() -> Particule {
		let pos = Point2D::new(rand::thread_rng().gen_range(0, (WIDTH*SQUARE_SIZE) as i32),rand::thread_rng().gen_range(0, (HEIGHT*SQUARE_SIZE) as i32));
		let speed = Vector2D::new(0.0 ,0.0);
		// let acc = Vector2D::new(0 ,0);

		Particule{
			_pos : pos,
			_pos_last : pos,
			_speed : speed,
			// _ac	c : acc
		}
	}

	pub fn step(&mut self, game :[f64; (WIDTH*HEIGHT) as usize]) {
		self._pos_last = self._pos;
		let x = if self._pos.x == (WIDTH*SQUARE_SIZE) as i32{

			self._pos.x / SQUARE_SIZE as i32 -1
		} else {
			self._pos.x / SQUARE_SIZE as i32 
		};
		let y = if self._pos.y == (HEIGHT*SQUARE_SIZE) as i32{

			self._pos.y / SQUARE_SIZE as i32 -1
		} else {
			self._pos.y / SQUARE_SIZE as i32
		};

		let index = (x + y * WIDTH as i32) as usize;
		// println!("{:?} {:?}", x, y);
		let acc = Vector2D::new(game[index].cos() * ACCEL, game[index].sin() * ACCEL);

		self._speed += acc;
		if self._speed.length() >  MAX_SPEED {
			self._speed = self._speed.normalize() * MAX_SPEED;
		}

		self._pos += self._speed.round().to_i32();

		if self._pos.x < 0 {
			self._pos.x += (WIDTH*SQUARE_SIZE) as i32;
			self._pos_last.x = (WIDTH*SQUARE_SIZE) as i32;
		} else if self._pos.x > (WIDTH*SQUARE_SIZE) as i32{
			self._pos.x -= (WIDTH*SQUARE_SIZE) as i32;
			self._pos_last.x = 0;

		}
		if self._pos.y < 0 {
			self._pos.y += (HEIGHT*SQUARE_SIZE) as i32;
			self._pos_last.y = (HEIGHT*SQUARE_SIZE) as i32;
		} else if self._pos.y > (HEIGHT*SQUARE_SIZE) as i32{
			self._pos.y -= (HEIGHT*SQUARE_SIZE) as i32;
			self._pos_last.y = 0;
		}

		if self._pos_last != self._pos{
			let diff_pos : Vector2D<f64>  = self._pos_last.to_f64() - self._pos.to_f64();

			// println!("{:?} {:?} {:?}", diff_pos, diff_pos_norm, diff_pos_norm.round().to_i32());
			self._pos_last += diff_pos.normalize().round().to_i32();
		}
			
	}

}