use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::{Arc, Mutex};

mod graph;
mod param_knob;

use crate::data::UIData;
use crate::TesticularDistortionParams;

use self::graph::{DistortionGraph, GraphBackground, QuadrantBorders};
use self::param_knob::ParamKnob;

#[derive(Lens)]
struct Data {
    params: Arc<TesticularDistortionParams>,
    ui_data: Arc<Mutex<UIData>>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 500))
}

pub(crate) fn create(
    params: Arc<TesticularDistortionParams>,
    ui_data: Arc<Mutex<UIData>>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        cx.add_theme(include_str!("editor/theme.css"));
        Data {
            params: params.clone(),
            ui_data: ui_data.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Testicular Distortion");
            HStack::new(cx, |cx| {
                ParamKnob::new(cx, Data::params, |p| &p.drive, false, false);
                ZStack::new(cx, |cx| {
                    GraphBackground::new(cx);
                    DistortionGraph::new(cx, Data::ui_data);
                    QuadrantBorders::new(cx);
                })
                .width(Pixels(300.0))
                .height(Pixels(300.0));
            });
        })
        .class("main");
    })
}
