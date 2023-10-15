use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Copy, Clone)]
pub enum DistortionAlgorithm {
    SoftClip,
    HardClip,
    Radial,
    Chomper,
}

impl DistortionAlgorithm {
    pub fn calculate(self, x: f32) -> f32 {
        match self {
            DistortionAlgorithm::SoftClip => soft_clip(x),
            DistortionAlgorithm::HardClip => hard_clip(x),
            DistortionAlgorithm::Radial => radial(x),
            DistortionAlgorithm::Chomper => chomper(x),
        }
    }
}

impl From<usize> for DistortionAlgorithm {
    fn from(id: usize) -> DistortionAlgorithm {
        Self::from_index(id)
    }
}

impl From<DistortionAlgorithm> for usize {
    fn from(t: DistortionAlgorithm) -> usize {
        t as usize
    }
}

#[inline]
pub fn soft_clip(mut x: f32) -> f32 {
    x = hard_clip(x);
    1.5 * (x - 1.0 / 3.0 * x.powf(3.0))
}

#[inline]
pub fn hard_clip(x: f32) -> f32 {
    x.max(-1.0).min(1.0)
}

#[inline]
pub fn radial(mut x: f32) -> f32 {
    if x == 0.0 {
        return 0.0;
    }
    x = hard_clip(x);
    x / x.abs() * (1.0 - (x - x / x.abs()).powf(2.0)).powf(0.5)
}

pub fn chomper(x: f32) -> f32 {
    hard_clip(1.5 * x - 0.7 * x.powf(3.0))
}
