use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

pub struct DistortionGraph;
pub struct Curve;

impl DistortionGraph {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self.build(cx, |cx| {
            Curve.build(cx, |_| {}).class("curve");
        })
    }
}

impl View for DistortionGraph {
    fn element(&self) -> Option<&'static str> {
        Some("distortion-graph")
    }
}

impl View for Curve {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let mut path = vg::Path::new();
        path.move_to(10.0, 10.0);
        path.line_to(25.0, 25.0);
        canvas.fill_path(&mut path, &vg::Paint::color(vg::Color::rgb(0, 0, 255)));
    }
}
