use crate::meta::samplerate;

use super::{partialbank::PartialBank, modules::partial::{shift::PartialShift, PartialFX}};
use nih_plug::{
	prelude::{NoteEvent, Smoother, SmoothingStyle},
	util::midi_note_to_freq,
};

pub struct Voice {
	active: bool,
	note: u8,
	bank: PartialBank,
	pub shift: PartialShift,
	// modules: Vec<Box<dyn PartialFX>>,
	declicked_amp: Smoother<f32>,
}

impl Voice {
	pub fn new() -> Self {
		Self {
			active: false,
			note: 0u8,
			bank: PartialBank::new(),
			shift: PartialShift::new(),
			// modules: Vec::<Box<dyn PartialFX>>::new(),
			declicked_amp: Smoother::new(SmoothingStyle::Linear(5.0)),
		}
	}

	pub fn tick(&mut self) -> f32 {
		self.shift.tick(&mut self.bank);
		match self.active {
			true => self.bank.tick() * self.declicked_amp.next(),
			false => 0f32,
		}
	}

	pub fn note_event(&mut self, event: NoteEvent<()>) {
		match event {
			NoteEvent::NoteOn { note, velocity, .. } => {
				self.trigger(note, velocity);
				self.active = true;
			}
			NoteEvent::NoteOff { note, .. } if note == self.note => {
				self.declicked_amp.set_target(samplerate(), 0.0);
				self.active = false;
			}
			NoteEvent::PolyPressure { note, pressure, .. } if note == self.note => {
				self.bank.set_fscale(pressure + 1f32);
				// self.declicked_amp.set_target(samplerate(), pressure);
			}
			_ => (),
		}
	}

	fn trigger(&mut self, note: u8, velocity: f32) {
		self.bank.init_phase();
		self.shift.reset(&mut self.bank);
		self.note = note;
		self.bank.set_fund(midi_note_to_freq(note));
		self.declicked_amp.set_target(samplerate(), velocity);
	}

	// pub fn get_midi_note(&self) -> u8 {
	// 	self.note
	// }
}
