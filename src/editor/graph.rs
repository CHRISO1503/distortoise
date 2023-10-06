use nih_plug::log::*;
use nih_plug::prelude::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg;
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;

use crate::algorithms::DistortionAlgorithm;

#[derive(Lens)]
pub struct DistortionGraph {
    param_base: ParamWidgetBase,
    algorithm: DistortionAlgorithm,
}
pub struct Curve {
    algorithm: DistortionAlgorithm,
}

const RESOLUTION: i32 = 100;

impl DistortionGraph {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        algorithm: DistortionAlgorithm,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
            algorithm,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params.clone(), params_to_param, move |cx, param_data| {
                // Binding::new(cx, algorithm, |cx, algorithm| {});
                Curve { algorithm }.build(cx, |_| {});
            }),
        )
    }
}

impl View for DistortionGraph {
    fn element(&self) -> Option<&'static str> {
        Some("distortion-graph")
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
