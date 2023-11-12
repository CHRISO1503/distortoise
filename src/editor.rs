use atomic_float::AtomicF32;
use nih_plug::prelude::Editor;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ResizeHandle;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod enum_button;
mod graph;
mod knob;
mod peak_meter;

use crate::data::UIData;
use crate::DistortoiseParams;

use self::enum_button::EnumButton;
use self::graph::{DistortionGraph, GraphBackground};
use self::knob::Knob;
use self::peak_meter::{PeakMeter, PeakMeterOutline};

pub const AFRICAN_TTF: &[u8] = include_bytes!("../assets/African-l6d.ttf");
pub const AFRICAN: &str = "African";

#[derive(Lens)]
struct Data {
    params: Arc<DistortoiseParams>,
    pre_peak_meter: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    ui_data: Arc<Mutex<UIData>>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (850, 450))
}

pub(crate) fn create(
    params: Arc<DistortoiseParams>,
    ui_data: Arc<Mutex<UIData>>,
    pre_peak_meter: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_fonts_mem(&[AFRICAN_TTF]);
        cx.add_theme(include_str!("editor/theme.css"));
        Data {
            params: params.clone(),
            pre_peak_meter: pre_peak_meter.clone(),
            peak_meter: peak_meter.clone(),
            ui_data: ui_data.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        ZStack::new(cx, |cx| {
            VStack::new(cx, |cx| {
                Label::new(cx, "Distortoise")
                    .font_family(vec![FamilyOwned::Name(String::from(AFRICAN))])
                    .class("title");
                HStack::new(cx, |cx| {
                    VStack::new(cx, |cx| {
                        HStack::new(cx, |cx| {
                            ZStack::new(cx, |cx| {
                                PeakMeter::new(
                                    cx,
                                    Data::pre_peak_meter.map(|peak_meter| {
                                        util::gain_to_db(peak_meter.load(Ordering::Relaxed))
                                    }),
                                    Some(Duration::from_millis(400)),
                                )
                                .class("peak-meter");
                                PeakMeterOutline::new(cx).class("peak-meter");
                            })
                            .class("peak-meter");
                            ZStack::new(cx, |cx| {
                                GraphBackground::new(cx);
                                DistortionGraph::new(cx, Data::ui_data);
                            })
                            .class("graph");
                            ZStack::new(cx, |cx| {
                                PeakMeter::new(
                                    cx,
                                    Data::peak_meter.map(|peak_meter| {
                                        util::gain_to_db(peak_meter.load(Ordering::Relaxed))
                                    }),
                                    Some(Duration::from_millis(400)),
                                )
                                .class("peak-meter");
                                PeakMeterOutline::new(cx).class("peak-meter");
                            })
                            .class("peak-meter");
                        })
                        .bottom(Pixels(10.0))
                        .space(Stretch(1.0));

                        HStack::new(cx, |cx| {
                            Knob::new(cx, Data::params, |p| &p.noise, false);
                            Knob::new(cx, Data::params, |p| &p.drive, false).class("drive");
                            Knob::new(cx, Data::params, |p| &p.mix, false).class("drive");
                            Knob::new(cx, Data::params, |p| &p.gain, false);
                        })
                        .top(Pixels(10.0));
                    });
                    HStack::new(cx, |cx| {
                        VStack::new(cx, |cx| {
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Softclip".to_string(),
                                0,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Hardclip".to_string(),
                                1,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Radial".to_string(),
                                2,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Chomper".to_string(),
                                3,
                                7,
                            );
                        });
                        VStack::new(cx, |cx| {
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Sine".to_string(),
                                4,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Stepper".to_string(),
                                5,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Humpback".to_string(),
                                6,
                                7,
                            );
                            EnumButton::new(
                                cx,
                                Data::params,
                                |p| &p.algorithm,
                                "Absolute".to_string(),
                                7,
                                7,
                            );
                        });
                    })
                    .class("algorithm-menu");
                })
                .class("body");
            });
        })
        .class("main");
    })
}
