use std::f32::consts::PI;

use nih_plug::prelude::Param;
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::vizia::vg::{Paint, Path, Solidity};
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;
use nih_plug_vizia::widgets::util::ModifiersExt;

static DRAG_SCALAR: f32 = 0.0042;
static MODIFIER_SCALAR: f32 = 0.04;

pub struct DragStatus {
    modified: bool,
    start_value: f32,
    start_y: f32,
}

pub struct Knob {
    param_base: ParamWidgetBase,
    is_dragging: bool,
    drag_status: Option<DragStatus>,
}

impl Knob {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        centered: bool,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
            is_dragging: false,
            drag_status: None,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params, params_to_param, move |cx, param_data| {
                VStack::new(cx, move |cx| {
                    ZStack::new(cx, |cx| {
                        KnobStatic::new(cx);
                        let param_lens = param_data
                            .clone()
                            .make_lens(|param| param.unmodulated_normalized_value());
                        Binding::new(cx, param_lens, move |cx, param_lens| {
                            KnobReactive::new(cx, param_lens.get_val(cx), centered);
                        })
                    })
                    .class("knob-graphic");
                    Label::new(cx, param_data.param().name())
                        .space(Stretch(1.0))
                        .top(Stretch(0.0));
                })
                .space(Stretch(1.0))
                .top(Stretch(0.0));
            }),
        )
    }
    fn reset_drag_status(&mut self, modified: bool, start_y: f32) {
        self.drag_status = Some(DragStatus {
            modified,
            start_value: self.param_base.modulated_normalized_value(),
            start_y,
        });
    }
}

impl View for Knob {
    fn element(&self) -> Option<&'static str> {
        Some("knob")
    }
    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                self.is_dragging = true;
                self.drag_status = Some(DragStatus {
                    modified: false,
                    start_value: self.param_base.modulated_normalized_value(),
                    start_y: cx.mouse.cursory,
                });
                cx.capture();
                cx.focus();
                cx.set_active(true);
                self.param_base.begin_set_parameter(cx);
                meta.consume();
            }
            WindowEvent::MouseUp(MouseButton::Left) => {
                if self.is_dragging {
                    self.is_dragging = false;
                    cx.release();
                    cx.set_active(false);
                    self.param_base.end_set_parameter(cx);
                    meta.consume();
                }
            }
            WindowEvent::MouseMove(_x, y) => {
                if self.is_dragging {
                    let delta_normal;
                    if cx.modifiers.shift() {
                        if self.drag_status.as_ref().unwrap().modified == false {
                            self.reset_drag_status(true, *y);
                        }
                        let prev_drag_status = self.drag_status.get_or_insert_with(|| DragStatus {
                            modified: false,
                            start_value: self.param_base.modulated_normalized_value(),
                            start_y: cx.mouse.cursory,
                        });
                        delta_normal =
                            (*y - prev_drag_status.start_y) * DRAG_SCALAR * MODIFIER_SCALAR;
                    } else {
                        if self.drag_status.as_ref().unwrap().modified == true {
                            self.reset_drag_status(false, *y);
                        }
                        delta_normal =
                            (*y - self.drag_status.as_ref().unwrap().start_y) * DRAG_SCALAR;
                    }
                    let new_normal = self.drag_status.as_ref().unwrap().start_value - delta_normal;
                    self.param_base.set_normalized_value(cx, new_normal);
                }
            }
            WindowEvent::MouseDoubleClick(button) if *button == MouseButton::Left => {
                self.is_dragging = false;
                self.param_base.begin_set_parameter(cx);
                self.param_base
                    .set_normalized_value(cx, self.param_base.default_normalized_value());
                self.param_base.end_set_parameter(cx);
                meta.consume();
            }
            _ => {}
        });
    }
}

pub struct KnobStatic {}

impl KnobStatic {
    pub fn new(cx: &mut Context) -> Handle<Self> {
        Self {}.build(cx, |_cx| ())
    }
}

impl View for KnobStatic {
    fn element(&self) -> Option<&'static str> {
        Some("knob-static")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let paint = Paint::color(cx.background_color().cloned().unwrap_or_default().into());
        let mut path = Path::new();
        path.circle(bounds.center().0, bounds.center().1, bounds.h / 2.0);
        canvas.fill_path(&mut path, &paint);
    }
}

pub struct KnobReactive {
    angle_start: f32,
    angle_end: f32,
    span: Units,
    normalized_value: f32,

    center: bool,
}

impl KnobReactive {
    pub fn new(cx: &mut Context, normalized_value: f32, center: bool) -> Handle<Self> {
        Self {
            angle_start: -150.0,
            angle_end: 150.0,
            span: Pixels(3.0),
            normalized_value,
            center,
        }
        .build(cx, |_cx| ())
    }
}

impl View for KnobReactive {
    fn element(&self) -> Option<&'static str> {
        Some("knob-reactive")
    }

    fn draw(&self, cx: &mut DrawContext, canvas: &mut Canvas) {
        let bounds = cx.bounds();
        let start = self.angle_start.to_radians() - PI / 2.0;
        let end = self.angle_end.to_radians() - PI / 2.0;
        let radius = bounds.h / 2.0;
        let span = self.span.value_or(radius, 0.0);
        let mut paint = Paint::color(cx.background_color().cloned().unwrap_or_default().into());
        paint.set_line_width(span);
        // Arc
        let mut arc_path = Path::new();
        if self.center {
            let center = -PI / 2.0;

            if self.normalized_value <= 0.5 {
                let current = self.normalized_value * 2.0 * (center - start) + start;
                arc_path.arc(
                    bounds.center().0,
                    bounds.center().1,
                    radius - span / 2.0,
                    center,
                    current,
                    Solidity::Solid,
                );
            } else {
                let current = (self.normalized_value * 2.0 - 1.0) * (end - center) + center;
                arc_path.arc(
                    bounds.center().0,
                    bounds.center().1,
                    radius - span / 2.0,
                    current,
                    center,
                    Solidity::Solid,
                );
            }
        } else {
            let current = self.normalized_value * (end - start) + start;
            arc_path.arc(
                bounds.center().0,
                bounds.center().1,
                radius - span / 2.0,
                current,
                start,
                Solidity::Solid,
            );
        }
        canvas.stroke_path(&mut arc_path, &paint);

        // Tick
        let mut tick_path = Path::new();
        let angle = start + (end - start) * self.normalized_value;
        tick_path.move_to(
            bounds.center().0 + angle.cos() * (radius * 0.5),
            bounds.center().1 + angle.sin() * (radius * 0.5),
        );
        tick_path.line_to(
            bounds.center().0 + angle.cos() * radius,
            bounds.center().1 + angle.sin() * radius,
        );
        canvas.stroke_path(&mut tick_path, &paint);
    }
}
