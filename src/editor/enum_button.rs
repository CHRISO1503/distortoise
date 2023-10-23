use nih_plug::prelude::{Param, Smoothable};
use nih_plug_vizia::vizia::prelude::*;
use nih_plug_vizia::widgets::param_base::ParamWidgetBase;

#[derive(Lens)]
pub struct EnumButton {
    param_base: ParamWidgetBase,
    option_id: i32,
    num_ids: i32,
}

impl EnumButton {
    pub fn new<L, Params, P, FMap>(
        cx: &mut Context,
        params: L,
        params_to_param: FMap,
        option_name: String,
        option_id: i32,
        num_ids: i32,
    ) -> Handle<Self>
    where
        L: Lens<Target = Params> + Clone,
        Params: 'static,
        P: Param + 'static,
        FMap: Fn(&Params) -> &P + Copy + 'static,
    {
        Self {
            param_base: ParamWidgetBase::new(cx, params.clone(), params_to_param),
            option_id,
            num_ids,
        }
        .build(
            cx,
            ParamWidgetBase::build_view(params.clone(), params_to_param, move |cx, _param_data| {
                Label::new(cx, &option_name)
                    .top(Stretch(1.0))
                    .bottom(Stretch(1.0))
                    .hoverable(false);
            }),
        )
        .checked(ParamWidgetBase::make_lens(
            params,
            params_to_param,
            move |param| {
                (param.modulated_normalized_value() * num_ids.to_f32() - option_id.to_f32()).abs()
                    < 0.5
            },
        ))
    }

    fn set_value(&self, cx: &mut EventContext) {
        self.param_base.begin_set_parameter(cx);
        self.param_base
            .set_normalized_value(cx, self.option_id as f32 / self.num_ids as f32);
        self.param_base.end_set_parameter(cx);
    }
}

impl View for EnumButton {
    fn element(&self) -> Option<&'static str> {
        Some("enum-button")
    }

    fn event(&mut self, cx: &mut EventContext, event: &mut Event) {
        event.map(|window_event, meta| match window_event {
            // We don't need special double and triple click handling
            WindowEvent::MouseDown(MouseButton::Left)
            | WindowEvent::MouseDoubleClick(MouseButton::Left)
            | WindowEvent::MouseTripleClick(MouseButton::Left) => {
                self.set_value(cx);
                meta.consume();
            }
            _ => {}
        });
    }
}
