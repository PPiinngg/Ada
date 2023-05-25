use nih_plug::prelude::NoteEvent;

use super::{
	consts::MAX_POLYPHONY,
	note::voice_allocator::{AdaNoteEvent, PolyVoiceAllocator},
	synth::{sinebank::SineBank, testsine::TestSine},
};

pub struct AdaState {
	polyvoice_allocator: PolyVoiceAllocator,
	polyvoices: Vec<SineBank>,
}

impl AdaState {
	pub fn new() -> Self {
		Self {
			polyvoice_allocator: PolyVoiceAllocator::new(),
			polyvoices: vec![SineBank::new(); MAX_POLYPHONY],
		}
	}

	pub fn render(&mut self) -> f32 {
		let mut sample = 0f32;
		for i in 0..MAX_POLYPHONY {
			sample += self.polyvoices[i].render();
		}
		return sample;
	}

	pub fn note_event(&mut self, event: NoteEvent<()>) {
		match self.polyvoice_allocator.parse_event(&event) {
			Some(ada_event) => match ada_event {
				AdaNoteEvent::Trigger {
					voice_idx,
					freq,
					vel,
				} => {
					self.polyvoices[voice_idx].set_fund(freq);
					self.polyvoices[voice_idx].reset_phases();
					// self.polyvoices[voice_idx].amp = vel;
				}

				AdaNoteEvent::Kill { voice_idx } => {
					// self.polyvoices[voice_idx].amp = 0f32;
					self.polyvoices[voice_idx].reset_phases();
					self.polyvoices[voice_idx].set_fund(0f32);
				}
			},

			None => {}
		}
	}

	pub fn reset(&mut self) {
		for i in 0..MAX_POLYPHONY {
			// self.polyvoices[i].amp = 0f32;
			self.polyvoices[i].reset_phases();
			self.polyvoices[i].set_fund(0f32);
		}
	}
}
