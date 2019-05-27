use noise::{NoiseFn, Perlin};
use arr_macro::arr;
use std::f64;
use std::thread;
extern crate crossbeam;

mod particules;

pub const SQUARE_SIZE: u32 = 10;
pub const WIDTH: u32 = 80;
pub const HEIGHT: u32 = 60;
// pub const SQUARE_SIZE: u32 = 40;
// pub const WIDTH: u32 = 20; // x
// pub const HEIGHT: u32 = 15; // y
pub const TIME_STEP: f64 = 0.015;
pub const STEP: f64 = 0.01;
pub const NB_PARTICULES: usize = 500;

#[derive(Copy, Clone)]
pub enum State {
    Paused,
    Playing,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mode {
    Art,
    Demo,
}

pub struct Flowfield {
	pub field : [f64; (WIDTH*HEIGHT) as usize],
	time : f64,
	perlin: Perlin,
	par: [particules::Particule; NB_PARTICULES],
	state: State,
	mode: Mode,
	// slc: Vec<[particules::Particule]>
}



impl Flowfield {
	pub fn new(_seed: u32) -> Flowfield {
		let mut t = 0.0;
    	let perlin = Perlin::new();
    	// perlin.set_seed(-seed);
    	// let mut _i = 0f64;
		// let mut field = arr![perlin.get([{_i+=1.0; _i/width as f64 *STEP}, _i%width as f64 *STEP, t]); WIDTH*HEIGHT];
		let mut field = [0.0; (WIDTH*HEIGHT) as usize];
		for _i in 0..WIDTH*HEIGHT {
			field[_i as usize] = perlin.get([(_i/WIDTH) as f64 *STEP, (_i%WIDTH) as f64 *STEP, t])* 4.0 * f64::consts::PI;
		}

		let mut parti = arr![particules::Particule::new(); 500];
		let mut slc = Vec::new();

		let bn_part_by_thread = NB_PARTICULES / 4;

		for i in 0..4 {
			slc.push(&mut parti[i*bn_part_by_thread..(i+1)*bn_part_by_thread]);
		}
		Flowfield {
			field : field,
			// field : [0.0; (WIDTH*HEIGHT) as usize],
			time : t,
			perlin : perlin,
			state : State::Paused,
			mode : Mode::Art,
			par : parti,
			// slice : slc
		}
	}

	pub fn update(&mut self) {
		self._update_perlin();
		self._update_particules();
	}

	fn _update_perlin(&mut self) {
		self.time += TIME_STEP;
		// println!("{:?}", self.time);
    	// let mut _i = 0f64;
		// self.field = arr![perlin.get([{_i+=1.0; _i/width as f64 *STEP}, _i%width as f64 *STEP, t]); WIDTH*HEIGHT];
		for _i in 0..WIDTH*HEIGHT {
			self.field[_i as usize] = self.perlin.get([(_i/WIDTH) as f64 *STEP, (_i%WIDTH) as f64 *STEP, self.time]) * 2.0 * f64::consts::PI;
		}
		// for _j in 0..10 {
		// 	print!("{:?} ", self.field[_j as usize]);
		// }
	}

	fn _update_particules(&mut self) {

		let bn_part_by_thread = self.par.len() / 4;

		crossbeam::scope(|scope| {
	        // The `collect` is important to eagerly start the threads!
	        let threads: Vec<_> = self.par
	            .chunks(bn_part_by_thread)
	            .map(|chunk| scope.spawn(move |_| {println!("{:?}", chunk.len());})
	            .collect();

	        let thr = threads.into_iter().map(|t| t.join());
	        println!("len    : {}", thr);
    	});
	}

    pub fn toggle_state(&mut self) {
	    self.state = match self.state {
	        State::Paused => State::Playing,
	        State::Playing => State::Paused,
	    }

	}

	pub fn toggle_mode(&mut self) {
	    self.mode = match self.mode {
	        Mode::Art => Mode::Demo,
	        Mode::Demo => Mode::Art,
	    }

	}
    
    pub fn state(&self) -> State {
	    self.state
	}

	pub fn mode(&self) -> Mode {
	    self.mode
	}
}

impl<'a> IntoIterator for &'a Flowfield {
    type Item = &'a particules::Particule;
    type IntoIter = ::std::slice::Iter<'a, particules::Particule>;
    fn into_iter(self) -> ::std::slice::Iter<'a, particules::Particule> {
        self.par.iter()
    }
}

fn worker(slice : &mut [particules::Particule]) {
	println!("hello {:?}", slice.len());
}