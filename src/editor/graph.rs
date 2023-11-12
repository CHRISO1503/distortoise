use std::sync::Arc;
use std::sync::Mutex;

use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

use crate::data::UIData;
pub struct DistortionGraph {
    ui_data: Arc<Mutex<UIData>>,
}
pub struct GraphBackground {}

impl DistortionGraph {
    pub fn new<LUIData>(cx: &mut Context, ui_data: LUIData) -> Handle<Self>
    where
        LUIData: Lens<Target = Arc<Mutex<UIData>>>,
    {
        Self {
            ui_data: ui_data.get(cx),
        }
        .build(cx, |_cx| ())
    }
}

impl View for DistortionGraph {
    fn element(&self) -> Option<&'static str> {
        Some("distortion-graph")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        const RESOLUTION: i32 = 1000;
        let data = self.ui_data.lock().unwrap();
        let algorithm = data.get_algorithm();
        let drive = data.get_drive();
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        let paint = vg::Paint::color(cx.background_color().cloned().unwrap_or_default().into());
        path.move_to(bounds.x, bounds.y + bounds.h);
        for i in 0..=RESOLUTION {
            let next_point = (
                bounds.x + i as f32 * bounds.w / RESOLUTION as f32,
                bounds.y
                    + 0.5
                        * bounds.h
                        * (1.0
                            - algorithm
                                .calculate(drive * (-1.0 + i as f32 * 2.0 / RESOLUTION as f32))),
            );
            path.line_to(next_point.0, next_point.1);
            path.move_to(next_point.0, next_point.1);
        }
        canvas.fill_path(&mut path, &paint);
    }
}

impl GraphBackground {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_cx| ())
    }
}

impl View for GraphBackground {
    fn element(&self) -> Option<&'static str> {
        Some("distortion-graph-background")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        // Background color
        let mut background_path = vg::Path::new();
        background_path.move_to(bounds.x, bounds.y);
        background_path.line_to(bounds.x + bounds.w, bounds.y);
        background_path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);
        background_path.line_to(bounds.x, bounds.y + bounds.h);
        background_path.close();
        canvas.fill_path(
            &mut background_path,
            &vg::Paint::color(vg::Color::rgb(255, 186, 73)),
        );

        // Draw grid for graph background
        let paint = vg::Paint::color(cx.background_color().cloned().unwrap_or_default().into());
        let mut path = vg::Path::new();
        for i in 1..10 {
            let increment = i as f32 * bounds.h / 10.0;
            path.move_to(bounds.x, bounds.y + increment);
            path.line_to(bounds.x + bounds.w, bounds.y + increment);
            path.move_to(bounds.x + increment, bounds.y);
            path.line_to(bounds.x + increment, bounds.y + bounds.h);
        }
        canvas.stroke_path(&mut path, &paint);

        // Draw quadrant borders
        let mut h_path = vg::Path::new();
        let mut v_path = vg::Path::new();
        h_path.move_to(bounds.x, bounds.y);
        h_path.line_to(bounds.x + bounds.w, bounds.y);

        h_path.move_to(bounds.x, bounds.y + 0.5 * bounds.h);
        h_path.line_to(bounds.x + bounds.w, bounds.y + 0.5 * bounds.h);

        h_path.move_to(bounds.x, bounds.y + bounds.h);
        h_path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);

        v_path.move_to(bounds.x, bounds.y);
        v_path.line_to(bounds.x, bounds.y + bounds.h);

        v_path.move_to(bounds.x + 0.5 * bounds.w, bounds.y);
        v_path.line_to(bounds.x + 0.5 * bounds.w, bounds.y + bounds.h);

        v_path.move_to(bounds.x + bounds.w, bounds.y);
        v_path.line_to(bounds.x + bounds.w, bounds.y + bounds.h);

        canvas.fill_path(&mut h_path, &paint);
        canvas.fill_path(&mut v_path, &paint);
    }
}
