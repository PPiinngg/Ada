use ada::voice::Voice;
use nih_plug::prelude::*;
use std::{f32::consts::TAU, sync::Arc};

mod ada;
mod meta;

struct Ada {
	params: Arc<AdaParams>,
	voice: Voice,
}

#[derive(Params)]
struct AdaParams {
	#[id = "gain"]
	pub gain: FloatParam,

	#[id = "phshift"]
	pub phshift: FloatParam,
}

impl Default for Ada {
	fn default() -> Self {
		Self {
			params: Arc::new(AdaParams::default()),
			voice: Voice::new(),
		}
	}
}

impl Default for AdaParams {
	fn default() -> Self {
		Self {
			gain: FloatParam::new(
				"Gain",
				-10.0,
				FloatRange::Linear {
					min: -30.0,
					max: 0.0,
				},
			)
			.with_smoother(SmoothingStyle::Linear(3.0))
			.with_step_size(0.01)
			.with_unit(" dB"),
			phshift: FloatParam::new(
				"Phase Shift",
				1.0,
				FloatRange::Linear { min: 0.0, max: TAU },
			)
			.with_smoother(SmoothingStyle::Linear(10.0)),
		}
	}
}

impl Plugin for Ada {
	const NAME: &'static str = "ada";
	const VENDOR: &'static str = "glyphli";
	const URL: &'static str = "https://glyphli.art";
	const EMAIL: &'static str = "hi@glyphli.art";

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
		AudioIOLayout {
			main_input_channels: None,
			main_output_channels: NonZeroU32::new(2),
			..AudioIOLayout::const_default()
		},
		AudioIOLayout {
			main_input_channels: None,
			main_output_channels: NonZeroU32::new(1),
			..AudioIOLayout::const_default()
		},
	];

	const MIDI_INPUT: MidiConfig = MidiConfig::Basic;
	const SAMPLE_ACCURATE_AUTOMATION: bool = true;

	type SysExMessage = ();
	type BackgroundTask = ();

	fn params(&self) -> Arc<dyn Params> {
		self.params.clone()
	}

	fn initialize(
		&mut self,
		_audio_io_layout: &AudioIOLayout,
		buffer_config: &BufferConfig,
		_context: &mut impl InitContext<Self>,
	) -> bool {
		unsafe { meta::SAMPLERATE_VAL = buffer_config.sample_rate };

		true
	}

	fn reset(&mut self) {}

	fn process(
		&mut self,
		buffer: &mut Buffer,
		_aux: &mut AuxiliaryBuffers,
		context: &mut impl ProcessContext<Self>,
	) -> ProcessStatus {
		let mut next_event = context.next_event();
		for (sample_id, channel_samples) in buffer.iter_samples().enumerate() {
			// Smoothing is optionally built into the parameters themselves
			let gain = self.params.gain.smoothed.next();
			let phshift = self.params.phshift.smoothed.next();
			self.voice.shift.shift = phshift;

			// iterate over note events
			while let Some(event) = next_event {
				if event.timing() > sample_id as u32 {
					break;
				}

				self.voice.note_event(event);

				next_event = context.next_event();
			}

			// This gain envelope prevents clicks with new notes and with released notes
			let out = self.voice.tick();

			for sample in channel_samples {
				*sample = out * util::db_to_gain_fast(gain);
			}
		}

		ProcessStatus::KeepAlive
	}
}

impl ClapPlugin for Ada {
	const CLAP_ID: &'static str = "com.glyphli.ada";
	const CLAP_DESCRIPTION: Option<&'static str> = Some("nuts-tacular additive synth");
	const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
	const CLAP_SUPPORT_URL: Option<&'static str> = None;
	const CLAP_FEATURES: &'static [ClapFeature] = &[
		ClapFeature::Instrument,
		ClapFeature::Synthesizer,
		ClapFeature::Stereo,
		ClapFeature::Mono,
		ClapFeature::Utility,
	];
}

impl Vst3Plugin for Ada {
	const VST3_CLASS_ID: [u8; 16] = *b"glyphliadaalpha!";
	const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
		&[Vst3SubCategory::Instrument, Vst3SubCategory::Synth];
}

nih_export_clap!(Ada);
nih_export_vst3!(Ada);
