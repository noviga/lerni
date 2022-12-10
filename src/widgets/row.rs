use std::rc::Rc;
use yew::{html::ChildrenRenderer, prelude::*};

use crate::widgets::{FromProperties, Widget, WidgetObject};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenRenderer<WidgetObject>,
    #[prop_or_default]
    pub x: i32,
    #[prop_or_default]
    pub y: i32,
    #[prop_or_default]
    pub width: i32,
    #[prop_or_default]
    pub height: i32,
}

/// Row of widgets.
#[derive(Clone)]
pub struct Row {
    props: Rc<Props>,
}

impl Component for Row {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Rc::new(ctx.props().clone()),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cols = p.children.len() as i32;
        let col_width = p.width / cols;

        html! {
            for p.children.iter().enumerate().map(|(i, mut item)| {
                let ox = p.x + col_width * i as i32;
                item.set_frame(ox, p.y, col_width, p.height);
                item
            })
        }
    }
}

impl Widget for Row {
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.x = x;
        p.y = y;
        p.width = width;
        p.height = height;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        html! {
            <Row x={ p.x } y={ p.y } width={ p.width } height={ p.height }>
                { for p.children.iter() }
            </Row>
        }
    }
}

impl FromProperties for Row {
    fn from_properties(props: Rc<Props>) -> Self {
        Self { props }
    }
}
