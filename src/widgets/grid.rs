use yew::prelude::*;

use super::common::{MultiWidget, Widget};

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub cols: usize,
}

/// Grid layout widget.
#[derive(Default)]
pub struct Grid {
    children: Vec<Box<dyn Widget>>,
    props: Props,
}

impl Widget for Grid {
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html {
        let cols = self.props.cols;
        let rows = self.props.rows;
        let col_width = width / cols;
        let row_height = height / rows;
        html! {
            <Grid { cols } { rows }>
                { for self.children().iter().enumerate().map(|(i, child)| {
                    let ox = x + (i % cols) * col_width;
                    let oy = y + (i / cols) * row_height;
                    child.render(ox, oy, col_width, row_height)
                }) }
            </Grid>
        }
    }
}

impl MultiWidget for Grid {
    fn children(&self) -> &[Box<dyn Widget>] {
        &self.children
    }
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget>> {
        &mut self.children
    }
}

impl Component for Grid {
    type Message = ();
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Default::default()
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let max = ctx.props().rows * ctx.props().cols;
        html!(for ctx.props().children.iter().take(max))
    }
}

impl Grid {
    /// Changes the column count and return boxed `Self`.
    pub fn cols(mut self: Box<Self>, cols: usize) -> Box<Self> {
        self.props.cols = cols;
        self
    }

    /// Changes the row count and return boxed `Self`.
    pub fn rows(mut self: Box<Self>, rows: usize) -> Box<Self> {
        self.props.rows = rows;
        self
    }
}

/// Creates a new `Grid` layout widget.
pub fn grid(children: Vec<Box<dyn Widget>>) -> Box<Grid> {
    Box::new(Grid {
        children,
        ..Default::default()
    })
}
