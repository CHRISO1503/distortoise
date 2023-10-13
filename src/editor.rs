use atomic_float::AtomicF32;
use nih_plug::prelude::Editor;
use nih_plug::util;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::ResizeHandle;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::atomic::Ordering;
use std::sync::{Arc, Mutex};
use std::time::Duration;

mod graph;
mod param_knob;
mod peak_meter;

use crate::data::UIData;
use crate::TesticularDistortionParams;

use self::graph::{DistortionGraph, GraphBackground, QuadrantBorders};
use self::param_knob::ParamKnob;
use self::peak_meter::{PeakMeter, PeakMeterOutline};

#[derive(Lens)]
struct Data {
    params: Arc<TesticularDistortionParams>,
    pre_peak_meter: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    ui_data: Arc<Mutex<UIData>>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (350, 600))
}

pub(crate) fn create(
    params: Arc<TesticularDistortionParams>,
    ui_data: Arc<Mutex<UIData>>,
    pre_peak_meter: Arc<AtomicF32>,
    peak_meter: Arc<AtomicF32>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_theme(include_str!("editor/theme.css"));
        Data {
            params: params.clone(),
            pre_peak_meter: pre_peak_meter.clone(),
            peak_meter: peak_meter.clone(),
            ui_data: ui_data.clone(),
        }
        .build(cx);

        ResizeHandle::new(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Testicular Distortion");
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
                    });
                    ZStack::new(cx, |cx| {
                        GraphBackground::new(cx);
                        DistortionGraph::new(cx, Data::ui_data);
                        QuadrantBorders::new(cx);
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
                    });
                });

                HStack::new(cx, |cx| {
                    ParamKnob::new(cx, Data::params, |p| &p.noise, false, false);
                    ParamKnob::new(cx, Data::params, |p| &p.drive, false, false).id("drive-knob");
                    ParamKnob::new(cx, Data::params, |p| &p.gain, false, false);
                });
            });
        })
        .class("main");
    })
}
