#[derive(Clone, Copy)]
pub struct TestSine {
	pub phasor: f32,
	pub init_phase: f32,

	pub freq: f32,
	pub step: f32,

	pub amp: f32,
}

impl TestSine {
	pub fn new() -> Self {
		Self {
			phasor: 0f32,
			init_phase: 0f32,

			freq: 0f32,
			step: 0f32,

			amp: 0f32,
		}
	}

	pub fn tick(&mut self) -> f32 {
		let sample: f32 = (self.phasor * std::f32::consts::TAU).sin() * self.amp;
		self.phasor += self.step;
		self.phasor = self.phasor.fract();
		return sample;
	}

	pub fn reset(&mut self) {
		self.phasor = self.init_phase;
		self.step = self.freq / super::super::meta::samplerate();
	}
}
