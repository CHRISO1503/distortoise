use data::UIData;
use nih_plug::prelude::*;
use rand::{rngs::StdRng, Rng, SeedableRng};
use std::sync::{Arc, Mutex};

mod algorithms;
mod data;
mod editor;
mod params;

use params::TesticularDistortionParams;

const PEAK_METER_DECAY_MS: f64 = 150.0;
const MAX_NOISE_VOLUME: f32 = 0.05;

struct TesticularDistortion {
    params: Arc<TesticularDistortionParams>,
    ui_data: Arc<Mutex<UIData>>,
    peak_meter_decay_weight: f32,
    pre_peak_meter: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    rng: StdRng,
}

impl Default for TesticularDistortion {
    fn default() -> Self {
        Self {
            params: Arc::new(TesticularDistortionParams::default()),
            ui_data: Arc::new(Mutex::new(UIData::default())),
            peak_meter_decay_weight: 1.0,
            peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            pre_peak_meter: Arc::new(AtomicF32::new(util::MINUS_INFINITY_DB)),
            rng: StdRng::seed_from_u64(0),
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
            self.pre_peak_meter.clone(),
            self.peak_meter.clone(),
            self.params.editor_state.clone(),
        )
    }

    fn initialize(
        &mut self,
        _audio_io_layout: &AudioIOLayout,
        buffer_config: &BufferConfig,
        _context: &mut impl InitContext<Self>,
    ) -> bool {
        self.peak_meter_decay_weight = 0.25f64
            .powf((buffer_config.sample_rate as f64 * PEAK_METER_DECAY_MS / 1000.0).recip())
            as f32;
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
            let noise = self.params.noise.smoothed.next();
            let algorithm = self.params.algorithm.value();
            let mut pre_amplitude = 0.0;
            let mut amplitude = 0.0;
            let num_samples = channel_samples.len();

            for sample in channel_samples {
                pre_amplitude += *sample;
                *sample *= 1.0 + self.rng.gen::<f32>() * MAX_NOISE_VOLUME * noise;
                *sample *= drive;
                *sample = algorithm.calculate(*sample);
                *sample *= gain;
                amplitude += *sample;
            }

            // Handle peak meter values
            if self.params.editor_state.is_open() {
                pre_amplitude = (pre_amplitude / num_samples as f32).abs();
                amplitude = (amplitude / num_samples as f32).abs();
                let current_pre_peak_meter = self
                    .pre_peak_meter
                    .load(std::sync::atomic::Ordering::Relaxed);
                let current_peak_meter = self.peak_meter.load(std::sync::atomic::Ordering::Relaxed);
                let new_pre_peak_meter = if pre_amplitude > current_pre_peak_meter {
                    pre_amplitude
                } else {
                    current_pre_peak_meter * self.peak_meter_decay_weight
                        + pre_amplitude * (1.0 - self.peak_meter_decay_weight)
                };
                let new_peak_meter = if amplitude > current_peak_meter {
                    amplitude
                } else {
                    current_peak_meter * self.peak_meter_decay_weight
                        + amplitude * (1.0 - self.peak_meter_decay_weight)
                };

                self.pre_peak_meter
                    .store(new_pre_peak_meter, std::sync::atomic::Ordering::Relaxed);
                self.peak_meter
                    .store(new_peak_meter, std::sync::atomic::Ordering::Relaxed)
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
