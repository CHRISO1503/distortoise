use std::f32::consts::PI;

use nih_plug::prelude::Enum;

#[derive(PartialEq, Enum, Copy, Clone)]
pub enum DistortionAlgorithm {
    SoftClip,
    HardClip,
    Radial,
    Chomper,
    Sine,
    Stepper,
    Humpback,
    Absolute,
}

impl DistortionAlgorithm {
    pub fn calculate(self, x: f32) -> f32 {
        match self {
            DistortionAlgorithm::SoftClip => soft_clip(x),
            DistortionAlgorithm::HardClip => hard_clip(x),
            DistortionAlgorithm::Radial => radial(x),
            DistortionAlgorithm::Chomper => chomper(x),
            DistortionAlgorithm::Sine => sine(x),
            DistortionAlgorithm::Stepper => stepper(x),
            DistortionAlgorithm::Humpback => humpback(x),
            DistortionAlgorithm::Absolute => absolute(x),
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

#[inline]
pub fn chomper(x: f32) -> f32 {
    hard_clip(1.5 * x - 0.7 * x.powf(3.0))
}

#[inline]
pub fn sine(x: f32) -> f32 {
    (PI / 2.0 * x).sin()
}

#[inline]
pub fn stepper(x: f32) -> f32 {
    hard_clip(0.5 * (x * (x * 2.0 * PI).cos() + x))
}

#[inline]
pub fn humpback(x: f32) -> f32 {
    hard_clip(0.14 * x.powf(5.0) - 1.15 * x.powf(3.0) + 1.9 * x)
}

#[inline]
pub fn absolute(x: f32) -> f32 {
    hard_clip(x.abs())
}
