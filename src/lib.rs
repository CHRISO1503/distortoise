use data::UIData;
use nih_plug::prelude::*;
use nih_plug_vizia::ViziaState;
use std::sync::{Arc, Mutex};

mod algorithms;
mod data;
mod editor;

use algorithms::DistortionAlgorithm;

struct TesticularDistortion {
    params: Arc<TesticularDistortionParams>,
    ui_data: Arc<Mutex<UIData>>,
}

#[derive(Params)]
struct TesticularDistortionParams {
    #[persist = "editor-state"]
    editor_state: Arc<ViziaState>,
    #[id = "algorithm"]
    pub algorithm: EnumParam<DistortionAlgorithm>,
    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "gain"]
    pub gain: FloatParam,
}

impl Default for TesticularDistortion {
    fn default() -> Self {
        Self {
            params: Arc::new(TesticularDistortionParams::default()),
            ui_data: Arc::new(Mutex::new(UIData::default())),
        }
    }
}

impl Default for TesticularDistortionParams {
    fn default() -> Self {
        Self {
            editor_state: editor::default_state(),

            algorithm: EnumParam::new("Algorithm", DistortionAlgorithm::SoftClip),

            drive: FloatParam::new(
                "Drive",
                1.0,
                FloatRange::Linear {
                    min: 1.0,
                    max: 10.0,
                },
            )
            .with_smoother(SmoothingStyle::Linear(50.0))
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(-30.0),
                    max: util::db_to_gain(30.0),
                    factor: FloatRange::gain_skew_factor(-30.0, 30.0),
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),
        }
    }
}

impl Plugin for TesticularDistortion {
    const NAME: &'static str = "Testicular Distortion";
    const VENDOR: &'static str = "Gayass Plugins";
    const URL: &'static str = env!("CARGO_PKG_HOMEPAGE");
    const EMAIL: &'static str = "your@email.com";

    const VERSION: &'static str = env!("CARGO_PKG_VERSION");

    const AUDIO_IO_LAYOUTS: &'static [AudioIOLayout] = &[AudioIOLayout {
        main_input_channels: NonZeroU32::new(2),
        main_output_channels: NonZeroU32::new(2),

        aux_input_ports: &[],
        aux_output_ports: &[],

        names: PortNames::const_default(),
    }];

    const MIDI_INPUT: MidiConfig = MidiConfig::None;
    const MIDI_OUTPUT: MidiConfig = MidiConfig::None;

    const SAMPLE_ACCURATE_AUTOMATION: bool = true;

    type SysExMessage = ();
    type BackgroundTask = ();

    fn params(&self) -> Arc<dyn Params> {
        self.params.clone()
    }

    fn editor(&mut self, _async_executor: AsyncExecutor<Self>) -> Option<Box<dyn Editor>> {
        editor::create(
            self.params.clone(),
            self.ui_data.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        _buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        true
    }

    fn reset(&mut self) {}

    fn process(
        &mut self,
        buffer: &mut Buffer,
        _aux: &mut AuxiliaryBuffers,
        _context: &mut impl ProcessContext<Self>,
    ) -> ProcessStatus {
        for channel_samples in buffer.iter_samples() {
            let gain = self.params.gain.smoothed.next();
            let drive = self.params.drive.smoothed.next();

            for sample in channel_samples {
                *sample *= drive;
                *sample = algorithms::soft_clip(*sample);
                *sample *= gain;
            }
        }
        if self.params.editor_state.is_open() {
            self.update_ui_data();
        }

        ProcessStatus::Normal
    }
}

impl TesticularDistortion {
    fn update_ui_data(&mut self) {
        let ui_data = self.ui_data.lock().unwrap();
        ui_data.set_drive(self.params.drive.smoothed.next());
        ui_data.set_algorithm(self.params.algorithm.value());
    }
}

impl ClapPlugin for TesticularDistortion {
    const CLAP_ID: &'static str = "com.your-domain.testicular-distortion";
    const CLAP_DESCRIPTION: Option<&'static str> = Some("Add torsion to dis");
    const CLAP_MANUAL_URL: Option<&'static str> = Some(Self::URL);
    const CLAP_SUPPORT_URL: Option<&'static str> = None;

    const CLAP_FEATURES: &'static [ClapFeature] = &[ClapFeature::AudioEffect, ClapFeature::Stereo];
}

impl Vst3Plugin for TesticularDistortion {
    const VST3_CLASS_ID: [u8; 16] = *b"testicledistort1";

    const VST3_SUBCATEGORIES: &'static [Vst3SubCategory] =
        &[Vst3SubCategory::Fx, Vst3SubCategory::Dynamics];
}

nih_export_clap!(TesticularDistortion);
nih_export_vst3!(TesticularDistortion);
