use yew::prelude::*;

use crate::layout::{SVG_HEIGHT, SVG_WIDTH};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| "none".to_string())]
    pub background: String,
}

#[function_component(Frame)]
pub fn frame(props: &Props) -> Html {
    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");
    html! {
        <svg viewBox={ view_box } class="has-ratio">
            <rect width="100%" height="100%" rx="10" fill={ props.background.clone() } />
            { for props.children.iter() }
        </svg>
    }
}
