use nih_plug::{prelude::*, util::MINUS_INFINITY_DB};
use nih_plug_vizia::ViziaState;
use std::sync::Arc;

use crate::{algorithms::DistortionAlgorithm, editor};

#[derive(Params)]
pub struct DistortoiseParams {
    #[persist = "editor-state"]
    pub editor_state: Arc<ViziaState>,
    #[id = "algorithm"]
    pub algorithm: EnumParam<DistortionAlgorithm>,
    #[id = "drive"]
    pub drive: FloatParam,
    #[id = "gain"]
    pub gain: FloatParam,
    #[id = "noise"]
    pub noise: FloatParam,
    #[id = "mix"]
    pub mix: FloatParam,
}

impl Default for DistortoiseParams {
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
            .with_value_to_string(formatters::v2s_f32_rounded(2)),

            gain: FloatParam::new(
                "Gain",
                util::db_to_gain(0.0),
                FloatRange::Skewed {
                    min: util::db_to_gain(MINUS_INFINITY_DB),
                    max: util::db_to_gain(0.0),
                    factor: 1.15,
                },
            )
            .with_smoother(SmoothingStyle::Logarithmic(50.0))
            .with_unit(" dB")
            .with_value_to_string(formatters::v2s_f32_gain_to_db(2))
            .with_string_to_value(formatters::s2v_f32_gain_to_db()),

            noise: FloatParam::new("Noise", 0.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0)),

            mix: FloatParam::new("Mix", 1.0, FloatRange::Linear { min: 0.0, max: 1.0 })
                .with_smoother(SmoothingStyle::Linear(50.0))
                .with_value_to_string(formatters::v2s_f32_rounded(2)),
        }
    }
}
