use super::super::consts::*;
use nih_plug::{prelude::NoteEvent, util::midi_note_to_freq};

const MIDI_MAX_ID: usize = 127usize;

pub enum AdaNoteEvent {
	Trigger {
		voice_idx: usize,
		freq: f32,
		vel: f32,
	},
	Kill {
		voice_idx: usize,
	},
}

// not quite round robin but good enough for now
pub struct PolyVoiceAllocator {
	/// voices[voice index] -> MIDI note ID
	voices: [Option<usize>; MAX_POLYPHONY],
	tracker: usize,
}

impl PolyVoiceAllocator {
	pub fn new() -> Self {
		Self {
			voices: [None; MAX_POLYPHONY],
			tracker: 0,
		}
	}


	pub fn parse_event(&mut self, event: &NoteEvent<()>) -> Option<AdaNoteEvent> {
		let mut result: Option<AdaNoteEvent> = None;
		match event {
			NoteEvent::NoteOn { note, velocity, .. } => {
				self.voices[self.tracker] = Some(*note as usize);
				result = Some(AdaNoteEvent::Trigger {
					voice_idx: self.tracker,
					freq: midi_note_to_freq(*note),
					vel: *velocity,
				});
				self.tracker += 1;
				self.tracker %= MAX_POLYPHONY;
			}

			NoteEvent::NoteOff { note, .. } => match self.get_voice_idx(&(*note as usize)) {
				Some(voice_idx) => {
                    self.voices[voice_idx] = None;
					result = Some(AdaNoteEvent::Kill {
						voice_idx: voice_idx,
					})
				}
				None => {}
			},

			_ => {}
		}
		return result;
	}


	pub fn get_voice_idx(&self, note_id: &usize) -> Option<usize> {
		for (i, voice) in self.voices.iter().enumerate() {
			match voice {
				Some(voice_note_id) => {
					if *voice_note_id == *note_id {
						return Some(i);
					}
				}
				None => {}
			}
		}

		return None;
	}


	pub fn get_note_id(&self, voice_idx: &usize) -> Option<usize> {
		for (i, voice) in self.voices.iter().enumerate() {
			match voice {
				Some(note_id) => {
					if i == *voice_idx {
						return Some(*note_id);
					}
				}
				None => {}
			}
		}

		return None;
	}
}
