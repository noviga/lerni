use yew::prelude::*;

use crate::widgets::Frame;

/// Grid layout widget.
#[function_component]
pub fn Grid(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let cols = props.cols;
    let rows = props.rows;
    let width = f.width / cols as i32;
    let height = f.height / rows as i32;

    let max = rows * cols;
    html! {
        for props.children.iter().take(max).enumerate().map(|(i, item)| {
            let x = f.x + width  * (i % cols) as i32;
            let y = f.y + height * (i / cols) as i32;
            let frame = Frame { x, y, width, height };
            html_nested! {
                <ContextProvider<Frame> context={ frame }>
                    { item }
                </ContextProvider<Frame>>
            }
        })
    }
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub cols: usize,
}
