use std::rc::Rc;
use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VChild};

use crate::widgets::{Widget2, WidgetObject};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenRenderer<WidgetObject>,
    #[prop_or_default]
    x: i32,
    #[prop_or_default]
    y: i32,
    #[prop_or_default]
    width: i32,
    #[prop_or_default]
    height: i32,
}

#[derive(Clone)]
pub struct Column {
    pub props: Rc<Props>,
}

impl Widget2 for Column {
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.x = x;
        p.y = y;
        p.width = width;
        p.height = height;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        let rows = p.children.len() as i32;
        let row_height = p.height / rows;

        html! {
            for p.children.iter().enumerate().map(|(i, mut item)| {
                let oy = p.y + row_height * i as i32;
                item.set_frame(p.x, oy, p.width, row_height);
                item
            })
        }
    }
}

impl Component for Column {
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

impl From<VChild<Column>> for WidgetObject {
    fn from(child: VChild<Column>) -> Self {
        Box::new(Column { props: child.props })
    }
}
