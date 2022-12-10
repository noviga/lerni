use std::rc::Rc;
use web_sys::SvgElement;
use yew::{html::ChildrenRenderer, prelude::*};

use crate::{
    properties::Color,
    widgets::{FromProperties, Widget, WidgetObject},
};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const POINTER_SIZE: i32 = 72;

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenRenderer<WidgetObject>,
    #[prop_or(WIDTH)]
    pub width: i32,
    #[prop_or(HEIGHT)]
    pub height: i32,
    #[prop_or_default]
    pub background: Color,
    #[prop_or_default]
    pub pointer: bool,
}

/// Slide widget.
#[derive(Clone, Default)]
pub struct Slide {
    props: Rc<Props>,
    svg_ref: NodeRef,
    pointer_x: i32,
    pointer_y: i32,
}

pub enum Msg {
    MovePointer { x: i32, y: i32 },
    HidePointer,
}

impl Component for Slide {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Rc::new(ctx.props().clone()),
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
        let p = ctx.props();
        let view_box = format!("0 0 {} {}", p.width, p.height);

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
                            { onmousemove } {onmouseleave} >
                            <rect width="100%" height="100%" rx="10" ry="10" fill={ p.background.to_string() } />
                            {
                                for p.children.iter().map(|mut item|{
                                    item.set_frame(0, 0, p.width, p.height);
                                    item
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
                <circle cx={ self.pointer_x.to_string() } cy={ self.pointer_y.to_string() }
                    r={ (POINTER_SIZE / 2).to_string() } fill="orange" opacity="0.75" pointer-events="none" />
            }
        } else {
            html_nested!()
        }
    }
}

impl Widget for Slide {
    fn set_frame(&mut self, _x: i32, _y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.width = width;
        p.height = height;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        html! {
            <Slide width={ p.width } height={ p.height } background={ p.background.clone() } pointer={ p.pointer }>
                { for p.children.iter() }
            </Slide>
        }
    }
}

impl FromProperties for Slide {
    fn from_properties(props: Rc<Props>) -> Self {
        Self {
            props,
            ..Default::default()
        }
    }
}
