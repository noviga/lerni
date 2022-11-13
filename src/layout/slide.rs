use yew::prelude::*;

use crate::layout::{SVG_HEIGHT, SVG_WIDTH};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| "none".to_string())]
    pub bgcolor: String,
}

/// Slide layout element.
#[function_component(Slide)]
pub fn slide(props: &Props) -> Html {
    let view_box = format!("0 0 {SVG_WIDTH} {SVG_HEIGHT}");

    html! {
        <div class="container pl-4 mt-4 pr-4">
            <div class="box">
                <figure class="image is-16by9">
                    <svg viewBox={ view_box } class="has-ratio">
                        <rect width="100%" height="100%" rx="10" ry="10" fill={ props.bgcolor.clone() } />
                        { for props.children.iter() }
                    </svg>
                </figure>
            </div>
        </div>
    }
}
