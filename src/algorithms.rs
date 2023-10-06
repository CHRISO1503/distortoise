use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Copy, Clone)]
pub enum DistortionAlgorithm {
    SoftClip,
    HardClip,
}

impl DistortionAlgorithm {
    pub fn calculate(self, x: f32) -> f32 {
        match self {
            DistortionAlgorithm::SoftClip => soft_clip(x),
            DistortionAlgorithm::HardClip => hard_clip(x),
        }
    }
}

pub fn soft_clip(mut x: f32) -> f32 {
    x = hard_clip(x);
    1.5 * (x - 1.0 / 3.0 * x.powf(3.0))
}

#[inline]
pub fn hard_clip(x: f32) -> f32 {
    x.max(-1.0).min(1.0)
}
