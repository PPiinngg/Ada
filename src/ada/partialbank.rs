use super::{consts::PARTIAL_COUNT, cordic::temp_rotate};
use crate::meta::samplerate;
use std::f32::consts::PI;

pub struct PartialBank {
	pub cos: Vec<f32>,
	pub sin: Vec<f32>,
	step: Vec<f32>,
	init_phase: Vec<f32>,

	pub ratio_tun: Vec<f32>,
	pub freq_tun: Vec<f32>,
	fscale: f32,
	fundamental: f32,

	amp: Vec<f32>,
}

impl PartialBank {
	pub fn new() -> Self {
		let mut ratios = Vec::<f32>::new();
		for i in 1..=PARTIAL_COUNT {
			ratios.push(i as f32);
		}

		let mut amps = Vec::<f32>::new();
		for i in 1..=PARTIAL_COUNT {
			amps.push(1f32 / (i as f32));
		}

		let new = Self {
			cos: vec![1f32; PARTIAL_COUNT],
			sin: vec![0f32; PARTIAL_COUNT],
			step: vec![0f32; PARTIAL_COUNT],
			init_phase: vec![0f32; PARTIAL_COUNT],

			ratio_tun: ratios,
			freq_tun: vec![0f32; PARTIAL_COUNT],
			fscale: 1f32,
			fundamental: 0f32,

			amp: amps,
		};

		return new;
	}

	pub fn set_fund(&mut self, freq: f32) {
		self.fundamental = freq;
	}

	pub fn set_fscale(&mut self, scale: f32) {
		self.fscale = scale;
	}

	pub fn init_phase(&mut self) {
		for i in 0..PARTIAL_COUNT {
			self.cos[i] = (self.init_phase[i] * 2f32 * PI).cos();
			self.sin[i] = (self.init_phase[i] * 2f32 * PI).sin();
		}
	}

	pub fn tick(&mut self) -> f32 {
		let mut result = 0f32;
		for i in 0..PARTIAL_COUNT {
			self.step[i] = ((self.fundamental
				* (((self.ratio_tun[i] - 1f32) * self.fscale) + 1f32))
				+ (self.freq_tun[i] * self.fscale))
				/ samplerate();

			temp_rotate(
				&(self.step[i] * 2f32 * PI),
				&mut self.cos[i],
				&mut self.sin[i],
			);

			let pre_nyq = self.sin[i] * self.amp[i];
			if self.step[i] < 0.5f32 {
				result += pre_nyq;
			}
		}
		return result;
	}
}
