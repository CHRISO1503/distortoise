use nih_plug::prelude::{Editor, GuiContext};
use nih_plug_iced::widgets as nih_widgets;
use nih_plug_iced::*;
use std::sync::Arc;

mod graph;

use crate::editor::graph::Graph;
use crate::TesticularDistortionParams;

pub(crate) fn default_state() -> Arc<IcedState> {
    IcedState::from_size(600, 400)
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
    graph_state: graph::State,
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
        let editor = TesticularDistortionEditor {
            params,
            context,
            graph_state: Default::default(),
        };
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
        //TODO: map graph to message once graph_state is non-empty
        Column::new().push(Graph::new(&mut self.graph_state)).into()
    }
}
