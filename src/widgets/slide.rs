use yew::prelude::*;

use crate::{properties::Color, widgets::Widget};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;

/// Slide widget.
#[derive(Default)]
pub struct Slide {
    content: Option<Box<dyn Widget>>,
    props: Props,
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub width: usize,
    pub height: usize,
    #[prop_or_default]
    pub background: Color,
}

impl Widget for Slide {
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html {
        html! {
            <Slide width={ width } height={ height } background={ self.props.background.clone() }>
                // TODO: Remove unwrap
                { self.content.as_ref().unwrap().render(x, y, width, height) }
            </Slide>
        }
    }
}

impl Component for Slide {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", ctx.props().width, ctx.props().height);

        html! {
            <div class="container pl-4 mt-4 pr-4">
                <div class="box">
                    <figure class="image is-16by9">
                        <svg viewBox={ view_box } class="has-ratio">
                            <rect width="100%" height="100%" rx="10" ry="10" fill={ ctx.props().background.to_string() } />
                            { for ctx.props().children.iter() }
                        </svg>
                    </figure>
                </div>
            </div>
        }
    }
}

impl Slide {
    /// Changes background color and returns boxed `Self`.
    pub fn background(mut self: Box<Self>, color: Color) -> Box<Self> {
        self.props.background = color;
        self
    }
}

impl From<Box<Slide>> for Html {
    fn from(value: Box<Slide>) -> Self {
        value.render(0, 0, WIDTH, HEIGHT)
    }
}

/// Creates a `Slide`.
pub fn slide(content: Box<dyn Widget>) -> Box<Slide> {
    Box::new(Slide {
        content: Some(content),
        ..Default::default()
    })
}
