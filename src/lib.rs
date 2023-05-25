#![feature(portable_simd)]

use ada::state::AdaState;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

mod ada;

/// A test tone generator that can either generate a sine wave based on the plugin's parameters or
/// based on the current MIDI input.
struct Ada {
	params: Arc<AdaParams>,
	state: AdaState,
}

#[derive(Params)]
struct AdaParams {
	#[persist = "editor-state"]
	editor_state: Arc<ViziaState>,

	#[id = "gain"]
	pub gain: FloatParam,
}

impl Default for Ada {
	fn default() -> Self {
		Self {
			params: Arc::new(AdaParams::default()),
			state: AdaState::new(),
		}
	}
}

impl Default for AdaParams {
	fn default() -> Self {
		Self {
			editor_state: ada::editor::default_state(),
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
		}
	}
}

impl Plugin for Ada {
	const NAME: &'static str = "Ada";
	const VENDOR: &'static str = "Glyphli & Friends";
	const URL: &'static str = "https://glyphli.art/"; // still have yet to do something with this url
	const EMAIL: &'static str = "hello@glyphli.art";

	const VERSION: &'static str = env!("CARGO_PKG_VERSION");

	const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[
		AudioIOLayout {
			// This is also the default and can be omitted here
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

	fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
		ada::editor::create(self.params.clone(), self.params.editor_state.clone())
	}

	fn initialize(
		&mut self,
		_audio_io_layout: &AudioIOLayout,
		buffer_config: &BufferConfig,
		_context: &mut impl InitContext<Self>,
	) -> bool {
		unsafe {
			ada::meta::SAMPLERATE_VAL = buffer_config.sample_rate;
		}

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

			// Act on the next MIDI event
			while let Some(event) = next_event {
				if event.timing() > sample_id as u32 {
					break;
				}
				self.state.note_event(event);
				next_event = context.next_event();
			}

			let mono_sample = self.state.render();

			for sample in channel_samples {
				*sample = mono_sample * util::db_to_gain_fast(gain);
			}
		}

		ProcessStatus::KeepAlive
	}
}

impl ClapPlugin for Ada {
	const CLAP_ID: &'static str = "com.glyphli.ada";
	const CLAP_DESCRIPTION: Option<&'static str> = Some("FOSS Additive Synth");
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
	const VST3_CLASS_ID: [u8; 16] = *b"glyphliadadevel!";
	const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] = &[
		Vst3SubCategory::Instrument,
		Vst3SubCategory::Synth,
		Vst3SubCategory::Stereo,
	];
}

nih_export_clap!(Ada);
nih_export_vst3!(Ada);
