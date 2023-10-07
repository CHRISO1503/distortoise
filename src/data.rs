use atomic_float::AtomicF32;
use nih_plug::prelude::*;
use paste::paste;
use std::sync::atomic::{AtomicUsize, Ordering};

use crate::algorithms::DistortionAlgorithm;

pub struct UIData {
    pub algorithm: AtomicUsize,
    pub drive: AtomicF32,
}

impl Default for UIData {
    fn default() -> Self {
        Self {
            algorithm: AtomicUsize::new(DistortionAlgorithm::SoftClip.into()),
            drive: AtomicF32::new(util::db_to_gain(0.0)),
        }
    }
}

macro_rules! get {
    ($name:ident $t:ty) => {
        paste! {
            pub fn [<get_ $name>](&self) -> $t {
                self.$name.load(Ordering::Relaxed).into()
            }
        }
    };
}
macro_rules! set {
    ($name:ident $t:ty) => {
        paste! {
            pub fn [<set_ $name>](&self, $name: $t) {
                self.$name.store($name.into(), Ordering::Relaxed);
            }
        }
    };
}

#[allow(dead_code)]
impl UIData {
    get!(algorithm DistortionAlgorithm);
    get!(drive f32);

    set!(algorithm DistortionAlgorithm);
    set!(drive f32);
}
