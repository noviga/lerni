use web_sys::SvgElement;
use yew::{prelude::*, ContextProvider};

use crate::{properties::Color, utils, widgets::Frame};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const POINTER_SIZE: i32 = 72;

/// Slide widget.
#[derive(Clone, Default)]
pub struct Slide {
    svg_ref: NodeRef,
    pointer_x: i32,
    pointer_y: i32,
    width: i32,
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(WIDTH)]
    pub width: i32,
    #[prop_or(HEIGHT)]
    pub height: i32,
    #[prop_or_default]
    pub background: Color,
    #[prop_or_default]
    pub pointer: bool,
    #[prop_or_default]
    pub onclick: Callback<(i32, i32)>,
}

#[derive(Clone, Copy)]
pub enum Msg {
    MovePointer { x: i32, y: i32 },
    HidePointer,
    Clicked { x: i32, y: i32 },
    Resize,
}

impl Component for Slide {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        utils::add_resize_handler(ctx.link(), Msg::Resize);
        Self {
            width: Self::calc_width(),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let p = ctx.props();
        match msg {
            Msg::MovePointer { x, y } => {
                if let Some(svg) = self.svg_ref.cast::<SvgElement>() {
                    self.pointer_x = x * WIDTH / svg.client_width();
                    self.pointer_y = y * HEIGHT / svg.client_height();
                }
                true
            }
            Msg::HidePointer => {
                self.pointer_x = 0;
                self.pointer_y = 0;
                true
            }
            Msg::Clicked { x, y } => {
                if let Some(svg) = self.svg_ref.cast::<SvgElement>() {
                    let x = x * WIDTH / svg.client_width();
                    let y = y * HEIGHT / svg.client_height();
                    p.onclick.emit((x, y));
                }
                false
            }
            Msg::Resize => {
                self.width = Self::calc_width();
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let view_box = format!("0 0 {} {}", p.width, p.height);

        let onmousemove = ctx.link().callback(|e: MouseEvent| Msg::MovePointer {
            x: e.offset_x(),
            y: e.offset_y(),
        });

        let onmouseleave = ctx.link().callback(|_| Msg::HidePointer);

        let onclick = ctx.link().callback(|e: MouseEvent| Msg::Clicked {
            x: e.offset_x(),
            y: e.offset_y(),
        });

        let frame = Frame {
            x: 0,
            y: 0,
            width: WIDTH,
            height: HEIGHT,
        };

        let style = if self.width > 0 {
            format!("max-width: {}px;", self.width)
        } else {
            ("max-width: 100%").to_string()
        };

        html! {
            <div { style } class="container pl-4 mt-4 pr-4">
                <div class="box">
                    <figure class="image is-16by9">
                        <svg viewBox={ view_box } class="has-ratio" ref={ &self.svg_ref }
                            { onmousemove } { onmouseleave } { onclick }>
                            <rect width="100%" height="100%" rx="10" ry="10" fill={ p.background.to_string() } />
                            {
                                for p.children.iter().map(|item|{
                                    html_nested! {
                                        <ContextProvider<Frame> context={ frame.clone() }>
                                            { item }
                                        </ContextProvider<Frame>>
                                    }
                                })
                            }
                            { self.pointer_view(p.pointer) }
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
                <circle cx={ self.pointer_x.to_string() } cy={ self.pointer_y.to_string() } style="filter: blur(2px);"
                    r={ (POINTER_SIZE / 2).to_string() } fill="orange" opacity="0.75" pointer-events="none" />
            }
        } else {
            html_nested!()
        }
    }

    fn calc_width() -> i32 {
        let elem = web_sys::window()
            .and_then(|win| win.document())
            .and_then(|doc| doc.document_element());
        if let Some(elem) = elem {
            let width = elem.client_width();
            let height = elem.client_height();
            width.min((height - 88) * 16 / 9)
        } else {
            0
        }
    }
}
