use nih_plug::prelude::Editor;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::*;
use nih_plug_vizia::{create_vizia_editor, ViziaState, ViziaTheming};
use std::sync::Arc;

mod graph;

use crate::TesticularDistortionParams;

use self::graph::DistortionGraph;

#[derive(Lens)]
struct Data {
    params: Arc<TesticularDistortionParams>,
}

impl Model for Data {}

pub(crate) fn default_state() -> Arc<ViziaState> {
    ViziaState::new(|| (800, 500))
}

pub(crate) fn create(
    params: Arc<TesticularDistortionParams>,
    editor_state: Arc<ViziaState>,
) -> Option<Box<dyn Editor>> {
    create_vizia_editor(editor_state, ViziaTheming::Custom, move |cx, _| {
        Data {
            params: params.clone(),
        }
        .build(cx);

        VStack::new(cx, |cx| {
            Label::new(cx, "Testicular Distortion");
            HStack::new(cx, |cx| {
                ParamSlider::new(cx, Data::params, |p| &p.drive);
                DistortionGraph::new(cx, Data::params, |p| &p.algorithm, params.algorithm.value())
                    .width(Percentage(50.0))
                    .height(Percentage(50.0));
            });
        });
    })
}
