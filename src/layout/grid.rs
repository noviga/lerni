use yew::prelude::*;

use crate::layout::{SVG_HEIGHT, SVG_WIDTH};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub columns: usize,
}

#[function_component(Grid)]
pub fn grid(props: &Props) -> Html {
    let n = props.columns.max(props.rows);

    let sx = 1.0 / n as f64;
    let sy = 1.0 / n as f64;

    let w = SVG_WIDTH / n;
    let h = SVG_HEIGHT / n;

    let dw = (SVG_WIDTH - w * props.columns) / (props.columns + 1);
    let dh = (SVG_HEIGHT - h * props.rows) / (props.rows + 1);

    html! {
        for props.children.iter().take(props.columns * props.rows).enumerate().map(|(i, child)| {
            let tx = dw + (i % props.columns) * (w + dw);
            let ty = dh + (i / props.columns) * (h + dh);
            let transform = format!("translate({tx} {ty}) scale({sx} {sy})");
            html_nested!(<g { transform }>{ child }</g>)
        })
    }
}
