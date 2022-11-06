use yew::prelude::*;

use crate::layout::{SVG_HEIGHT, SVG_WIDTH};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub cols: usize,
}

/// Grid layout element.
#[function_component(Grid)]
pub fn grid(props: &Props) -> Html {
    let n = props.cols.max(props.rows);

    let sx = 1.0 / n as f64;
    let sy = 1.0 / n as f64;

    let w = SVG_WIDTH / n;
    let h = SVG_HEIGHT / n;

    let dw = (SVG_WIDTH - w * props.cols) / (props.cols + 1);
    let dh = (SVG_HEIGHT - h * props.rows) / (props.rows + 1);

    html! {
        for props.children.iter().take(props.cols * props.rows).enumerate().map(|(i, child)| {
            let tx = dw + (i % props.cols) * (w + dw);
            let ty = dh + (i / props.cols) * (h + dh);
            let transform = format!("translate({tx} {ty}) scale({sx} {sy})");
            html_nested!(<g { transform }>{ child }</g>)
        })
    }
}
