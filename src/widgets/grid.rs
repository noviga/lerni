use std::rc::Rc;
use yew::{html::ChildrenRenderer, prelude::*, virtual_dom::VChild};

use crate::widgets::{Widget, WidgetObject};

/// Grid layout widget.
#[derive(Clone, Default)]
pub struct Grid {
    props: Rc<Props>,
}

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
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub cols: usize,
}

impl Component for Grid {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Rc::new(ctx.props().clone()),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let cols = p.cols;
        let rows = p.rows;
        let col_width = p.width / cols as i32;
        let row_height = p.height / rows as i32;

        let max = rows * cols;
        html! {
            for p.children.iter().take(max).enumerate().map(|(i, mut item)| {
                let ox = p.x + col_width * (i % cols) as i32;
                let oy = p.y + row_height * (i / cols)  as i32 ;
                item.set_frame(ox, oy, col_width, row_height);
                item
            })
        }
    }
}

impl Widget for Grid {
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
            <Grid x={ p.x } y={ p.y } width={ p.width } height={ p.height } cols={ p.cols } rows={ p.rows }>
                { for p.children.iter() }
            </Grid>
        }
    }
}

impl From<VChild<Grid>> for WidgetObject {
    fn from(child: VChild<Grid>) -> Self {
        Box::new(Grid { props: child.props })
    }
}
