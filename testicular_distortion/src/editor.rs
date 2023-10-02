use atomic_float::AtomicF32;
use nih_plug::prelude::{util, Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;

use crate::TesticularDistortionParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(200, 150)
}

pub(crate) fn create(
    params: Arc<TesticularDistortionParams>,
    editor_state: Arc<IcedState>,
) -> Option<Box<dyn Editor>> {
    create_iced_editor::<TesticularDistortionEditor>(editor_state, params)
}

struct TesticularDistortionEditor {
    params: Arc<TesticularDistortionParams>,
    context: Arc<dyn GuiContext>,
}

#[derive(Debug, Clone, Copy)]
enum Message {
    /// Update a parameter's value.
    ParamUpdate(nih_widgets::ParamMessage),
}

impl IcedEditor for TesticularDistortionEditor {
    type Executor = executor::Default;
    type Message = Message;
    type InitializationFlags = Arc<TesticularDistortionParams>;

    fn new(
        params: Self::InitializationFlags,
        context: Arc<dyn GuiContext>,
    ) -> (Self, Command<Self::Message>) {
        let editor = TesticularDistortionEditor { params, context };
        (editor, Command::none())
    }

    fn context(&self) -> &dyn GuiContext {
        self.context.as_ref()
    }

    fn update(
        &mut self,
        _window: &mut WindowQueue,
        message: Self::Message,
    ) -> Command<Self::Message> {
        match message {
            Message::ParamUpdate(message) => self.handle_param_message(message),
        }

        Command::none()
    }

    fn view(&mut self) -> Element<'_, Self::Message> {
        Column::new().into()
    }

    fn background_color(&self) -> nih_plug_iced::Color {
        nih_plug_iced::Color {
            r: 0.98,
            g: 0.98,
            b: 0.98,
            a: 1.0,
        }
    }
}
