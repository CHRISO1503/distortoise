use nih_plug_iced::backend::Renderer;
use nih_plug_iced::renderer::Renderer as GraphicsRenderer;
use nih_plug_iced::*;
use std::marker::PhantomData;

pub struct Graph<'a, Message> {
    state: &'a mut State,
    width: Length,
    height: Length,
    _phantom: PhantomData<Message>,
}

#[derive(Debug, Default)]
pub struct State {}

impl<'a, Message> Graph<'a, Message> {
    pub fn new(state: &'a mut State) -> Self {
        Self {
            state,
            width: Length::Fill,
            height: Length::Fill,
            _phantom: PhantomData,
        }
    }
}

impl<'a, Message> Widget<Message, Renderer> for Graph<'a, Message>
where
    Message: Clone,
{
    fn width(&self) -> Length {
        self.width
    }

    fn height(&self) -> Length {
        self.height
    }

    fn layout(&self, _renderer: &Renderer, limits: &layout::Limits) -> layout::Node {
        let limits = limits.width(self.width).height(self.height);
        let size = limits.resolve(Size::ZERO);

        layout::Node::new(size)
    }

    fn draw(
        &self,
        renderer: &mut Renderer,
        _style: &renderer::Style,
        layout: Layout<'_>,
        _cursor_position: Point,
        _viewport: &Rectangle,
    ) {
        let bounds = layout.bounds();
        renderer.fill_quad(
            renderer::Quad {
                bounds: bounds,
                border_radius: 0.0,
                border_width: 0.0,
                border_color: Color::TRANSPARENT,
            },
            Background::Color(Color::BLACK),
        );
    }
}

impl<'a, Message> From<Graph<'a, Message>> for Element<'a, Message>
where
    Message: 'a + Clone,
{
    fn from(widget: Graph<'a, Message>) -> Self {
        Element::new(widget)
    }
}
