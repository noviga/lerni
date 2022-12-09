use std::rc::Rc;
use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VChild};

use crate::{
    properties::Color,
    widgets::{Widget2, WidgetObject},
};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

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
pub struct Slide2 {
    props: Rc<Props>,
}

pub enum Msg {
    MovePointer { x: i32, y: i32 },
    HidePointer,
}

impl Widget2 for Slide2 {
    fn set_frame(&mut self, _x: i32, _y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.width = width;
        p.height = height;
    }

    fn render(&self) -> Html {
        let p = &self.props;

        let view_box = format!("0 0 {} {}", p.width, p.height);

        html! {
            <div class="container pl-4 mt-4 pr-4">
                <div class="box">
                    <figure class="image is-16by9">
                        <svg viewBox={ view_box } class="has-ratio">
                            <rect width="100%" height="100%" rx="10" ry="10" fill={ p.background.to_string() } />
                            {
                                for p.children.iter().map(|mut item|{
                                    item.set_frame(0, 0, p.width, p.height);
                                    item
                                })
                            }
                        </svg>
                    </figure>
                </div>
            </div>
        }
    }
}

impl Component for Slide2 {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Rc::new(ctx.props().clone()),
        }
    }

    fn view(&self, _ctx: &Context<Self>) -> Html {
        self.render()
    }
}

impl From<VChild<Slide2>> for WidgetObject {
    fn from(child: VChild<Slide2>) -> Self {
        Box::new(Slide2 { props: child.props })
    }
}
