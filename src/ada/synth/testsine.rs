use std::f32::consts::TAU;

use crate::ada::util::cordic::scalar::temp_rotate;

#[derive(Clone, Copy)]
pub struct TestSine {
	// pub phasor: f32,
	pub cos: f32,
	pub sin: f32,
	pub init_phase: f32,

	pub freq: f32,
	pub step: f32,

	pub amp: f32,
}

impl TestSine {
	pub fn new() -> Self {
		Self {
			cos: 1f32,
			sin: 0f32,
			init_phase: 0f32,

			freq: 0f32,
			step: 0f32,

			amp: 0f32,
		}
	}

	pub fn tick(&mut self) {
		temp_rotate(&self.step, &mut self.cos, &mut self.sin);
	}

	pub fn render(&mut self) -> f32 {
		self.tick();
		self.sin * self.amp
	}

	pub fn calc_step(&mut self) {
		self.step = (self.freq / super::super::meta::samplerate()) * TAU;
	}

	pub fn reset(&mut self) {
		self.calc_step();
		self.cos = 1f32;
		self.sin = 0f32;
		temp_rotate(&self.init_phase, &mut self.cos, &mut self.sin);
	}
}
