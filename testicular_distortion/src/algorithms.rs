use std::f32::consts::E;

pub fn soft_clip(x: f32) -> f32 {
    x / x.abs() * (1.0 - E.powf(-x.powf(2.0) / x.abs()))
}
