use leptos::{component, view, IntoView};

use crate::use_frame;

#[component]
pub fn Space(#[prop(optional, into)] image: String) -> impl IntoView {
    let f = use_frame();

    view! {
        <image
            href=image
            x=f.x
            y=f.y
            width=f.width
            height=f.height
            preserveAspectRatio="meet xMaxYMax"
        ></image>
    }
}
