use std::sync::Arc;
use std::sync::Mutex;

use nih_plug::log::*;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;

use crate::algorithms::DistortionAlgorithm;
use crate::data::UIData;

pub struct DistortionGraph {
    ui_data: Arc<Mutex<UIData>>,
}
pub struct Curve {
    algorithm: DistortionAlgorithm,
}

const RESOLUTION: i32 = 100;

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
        let data = self.ui_data.lock().unwrap();
        let algorithm = data.get_algorithm();
        let drive = data.get_drive();
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        path.move_to(bounds.x, bounds.y + bounds.h);
        debug!(
            "bounds.x: {:?}, bounds.w: {:?}, bounds.y:{:?}, bounds.h: {:?}",
            bounds.x, bounds.w, bounds.y, bounds.h
        );
        for i in 0..RESOLUTION {
            let next_point = (
                bounds.x + i as f32 * bounds.w / RESOLUTION as f32,
                bounds.y
                    + 0.5
                        * bounds.h
                        * (1.0
                            - algorithm
                                .calculate(drive * (-1.0 + i as f32 * 2.0 / RESOLUTION as f32))),
            );
            debug!("next point: {:?}", next_point);
            debug!(
                "algorithm({:?}): {:?}",
                -1.0 + i as f32 * 2.0 / RESOLUTION as f32,
                algorithm.calculate(-1.0 + i as f32 * 2.0 / RESOLUTION as f32)
            );
            path.line_to(next_point.0, next_point.1);
            path.move_to(next_point.0, next_point.1);
        }
        canvas.fill_path(&mut path, &vg::Paint::color(vg::Color::rgb(0, 0, 255)));
    }
}

impl View for Curve {
    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let mut path = vg::Path::new();
        path.move_to(bounds.x, bounds.y + bounds.h);
        debug!(
            "bounds.x: {:?}, bounds.w: {:?}, bounds.y:{:?}, bounds.h: {:?}",
            bounds.x, bounds.w, bounds.y, bounds.h
        );
        for i in 0..RESOLUTION {
            let next_point = (
                bounds.x + i as f32 * bounds.w / RESOLUTION as f32,
                bounds.y
                    + 0.5
                        * bounds.h
                        * (1.0
                            - self
                                .algorithm
                                .calculate(-1.0 + i as f32 * 2.0 / RESOLUTION as f32)),
            );
            debug!("next point: {:?}", next_point);
            debug!(
                "algorithm({:?}): {:?}",
                -1.0 + i as f32 * 2.0 / RESOLUTION as f32,
                self.algorithm
                    .calculate(-1.0 + i as f32 * 2.0 / RESOLUTION as f32)
            );
            path.line_to(next_point.0, next_point.1);
            path.move_to(next_point.0, next_point.1);
        }
        canvas.fill_path(&mut path, &vg::Paint::color(vg::Color::rgb(0, 0, 255)));
    }
}
