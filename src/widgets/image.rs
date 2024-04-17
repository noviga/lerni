use leptos::*;

use crate::use_frame;

#[component]
pub fn Image(
    #[prop(into)] src: MaybeSignal<String>,
    #[prop(default = true.into(), into)] visible: MaybeSignal<bool>,
    #[prop(default = "all .3s".to_string(), into)] transition: String,
) -> impl IntoView {
    let f = use_frame();

    view! {
        <g
            style:opacity=move || if visible.get() { 1 } else { 0 }
            style:visibility=move || { if visible.get() { "visible" } else { "hidden" } }
            style:transition=transition
        >
            <image
                href=src
                x=f.x
                y=f.y
                width=f.width
                height=f.height
                preserveAspectRatio="xMidYMid meet"
            ></image>
        </g>
    }
}
