use web_sys::SvgElement;
use yew::prelude::*;

use crate::{properties::Color, widgets::Widget};

const WIDTH: usize = 1920;
const HEIGHT: usize = 1080;
const POINTER_SIZE: i32 = 72;

/// Slide widget.
#[derive(Default)]
pub struct Slide {
    content: Option<Box<dyn Widget>>,
    props: Props,
    svg_ref: NodeRef,
    pointer_x: i32,
    pointer_y: i32,
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    pub width: usize,
    pub height: usize,
    #[prop_or_default]
    pub background: Color,
    #[prop_or_default]
    pub pointer: bool,
}

pub enum Msg {
    MovePointer { x: i32, y: i32 },
    HidePointer,
}

impl Widget for Slide {
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html {
        html! {
            <Slide width={ width } height={ height } background={ self.props.background.clone() }
                pointer={ self.props.pointer }>
                // TODO: Remove unwrap
                { self.content.as_ref().unwrap().render(x, y, width, height) }
            </Slide>
        }
    }
}

impl Component for Slide {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            ..Default::default()
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::MovePointer { x, y } => {
                if let Some(svg) = self.svg_ref.cast::<SvgElement>() {
                    self.pointer_x = x * WIDTH as i32 / svg.client_width();
                    self.pointer_y = y * HEIGHT as i32 / svg.client_height();
                }
                true
            }
            Msg::HidePointer => {
                self.pointer_x = 0;
                self.pointer_y = 0;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let view_box = format!("0 0 {} {}", ctx.props().width, ctx.props().height);

        let onmousemove = ctx.link().callback(|e: MouseEvent| Msg::MovePointer {
            x: e.offset_x(),
            y: e.offset_y(),
        });

        let onmouseleave = ctx.link().callback(|_| Msg::HidePointer);

        html! {
            <div class="container pl-4 mt-4 pr-4">
                <div class="box">
                    <figure class="image is-16by9">
                        <svg viewBox={ view_box } class="has-ratio" ref={ &self.svg_ref }
                            { onmousemove } {onmouseleave}>
                            <rect width="100%" height="100%" rx="10" ry="10" fill={ ctx.props().background.to_string() } />
                            { for ctx.props().children.iter() }
                            { self.pointer_view(ctx.props().pointer) }
                        </svg>
                    </figure>
                </div>
            </div>
        }
    }
}

impl Slide {
    fn pointer_view(&self, pointer: bool) -> Html {
        if pointer && self.pointer_x > 0 && self.pointer_y > 0 {
            html_nested! {
                <circle cx={ self.pointer_x.to_string() } cy={ self.pointer_y.to_string() }
                    r={ (POINTER_SIZE / 2).to_string() } fill="orange" opacity="0.75" pointer-events="none" />
            }
        } else {
            html_nested!()
        }
    }
}

impl Slide {
    /// Changes background color and returns boxed `Self`.
    pub fn background(mut self: Box<Self>, color: Color) -> Box<Self> {
        self.props.background = color;
        self
    }

    /// Changes the pointer visibility and returns boxed `Self`.
    pub fn pointer(mut self: Box<Self>, flag: bool) -> Box<Self> {
        self.props.pointer = flag;
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
