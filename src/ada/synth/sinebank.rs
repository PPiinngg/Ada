use crate::ada::{
	consts::{PARTIAL_COUNT, SIMD_SIZE},
	meta::samplerate,
	types::SimdFloats,
	util::cordic::simd::{cordic_rotate, temp_rotate},
};

pub const PARTIAL_VEC_SIZE: usize = PARTIAL_COUNT / SIMD_SIZE;

#[derive(Clone)]
pub struct SineBank {
	pub cos: Vec<SimdFloats>,
	pub sin: Vec<SimdFloats>,
	pub phase: Vec<SimdFloats>,

	step: Vec<SimdFloats>,
	pub freq: Vec<SimdFloats>,
	pub ratio: Vec<SimdFloats>,
	fund: SimdFloats,

	pub amp: Vec<SimdFloats>,
}

impl SineBank {
	pub fn new() -> Self {
		let mut ratios: Vec<SimdFloats> = vec![SimdFloats::default(); PARTIAL_VEC_SIZE];
		for i1 in 0..PARTIAL_VEC_SIZE {
			for i2 in 0..SIMD_SIZE {
				ratios[i1][i2] = (i1 + i2 + 1usize) as f32;
			}
		}

		let mut amps: Vec<SimdFloats> = vec![SimdFloats::default(); PARTIAL_VEC_SIZE];
		for i1 in 0..PARTIAL_VEC_SIZE {
			for i2 in 0..SIMD_SIZE {
				amps[i1][i2] = 1f32 / ((i1 + i2 + 1) as f32);
			}
		}

		Self {
			cos: vec![SimdFloats::splat(1f32); PARTIAL_VEC_SIZE],
			sin: vec![SimdFloats::splat(0f32); PARTIAL_VEC_SIZE],
			phase: vec![SimdFloats::splat(0f32); PARTIAL_VEC_SIZE],

			step: vec![SimdFloats::splat(0f32); PARTIAL_VEC_SIZE],
			freq: vec![SimdFloats::splat(0f32); PARTIAL_VEC_SIZE],
			ratio: ratios,
			fund: SimdFloats::splat(0f32),

			amp: amps,
		}
	}

	pub fn reset_phases(&mut self) {
		let init_cos = SimdFloats::splat(1f32);
		let init_sin = SimdFloats::splat(0f32);
		for i in 0..PARTIAL_VEC_SIZE {
			self.cos[i] = init_cos;
			self.sin[i] = init_sin;
			cordic_rotate(&self.phase[i], &mut self.cos[i], &mut self.sin[i]);
		}
	}

	#[inline]
	pub fn set_fund(&mut self, new_fund: f32) {
		self.fund = SimdFloats::splat(new_fund);
		self.calc_step();
	}

	#[inline]
	pub fn calc_step(&mut self) {
		let samplerate = SimdFloats::splat(samplerate());
		for i in 0..PARTIAL_VEC_SIZE {
			self.step[i] = (((self.fund * self.ratio[i]) + self.freq[i]) / samplerate);
		}
	}

	#[inline]
	pub fn tick(&mut self) {
		for i in 0..PARTIAL_VEC_SIZE {
			temp_rotate(&self.step[i], &mut self.cos[i], &mut self.sin[i]);
		}
	}

	pub fn render(&mut self) -> f32 {
		self.tick();

		let mut result = SimdFloats::default();
		for i in 0..PARTIAL_VEC_SIZE {
			result += self.sin[i];
		}
		result.as_array().into_iter().sum()
	}
}
