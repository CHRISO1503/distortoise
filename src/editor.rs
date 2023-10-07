use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::{Arc, Mutex};

mod graph;

use crate::data::UIData;
use crate::TesticularDistortionParams;

use self::graph::DistortionGraph;

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
        Data {
            params: params.clone(),
            ui_data: ui_data.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Testicular Distortion");
            HStack::new(cx, |cx| {
                ParamSlider::new(cx, Data::params, |p| &p.drive);
                DistortionGraph::new(cx, Data::ui_data)
                    .width(Percentage(50.0))
                    .height(Percentage(50.0));
            });
        });
    })
}
