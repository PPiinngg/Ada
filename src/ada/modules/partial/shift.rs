use super::PartialFX;
use crate::ada::{consts::PARTIAL_COUNT, cordic::temp_rotate, partialbank::PartialBank};

pub struct PartialShift {
	pub shift: f32,
	prev: f32,
}

impl PartialShift {
	pub fn new() -> Self {
		Self {
			shift: 0f32,
			prev: 0f32,
		}
	}
}

impl PartialFX for PartialShift {
	fn tick(&mut self, bank: &mut PartialBank) {
		let shift = self.shift - self.prev;

		for i in 0..PARTIAL_COUNT {
			temp_rotate(&shift, &mut bank.cos[i], &mut bank.sin[i]);
		}

		self.prev = self.shift;
	}

	fn reset(&mut self, _: &mut PartialBank) {
		self.prev = 0f32;
	}
}
