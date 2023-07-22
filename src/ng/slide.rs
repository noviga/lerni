use leptos::*;

use crate::ng::{Color, Frame};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

#[component]
pub fn Slide(
    cx: Scope,
    #[prop(default=WIDTH)] width: i32,
    #[prop(default=HEIGHT)] height: i32,
    #[prop(optional)] background_color: Color,
    #[prop(optional)] _background_image: String,
    #[prop(optional)] _pointer: bool,
    #[prop(optional)] _blur: bool,
    #[prop(default = 15)] _blur_radius: i32,
    children: Children,
) -> impl IntoView {
    let frame = Frame {
        width,
        height,
        ..Default::default()
    };
    provide_context(cx, frame);

    let view_box = format!("0 0 {width} {height}");

    view! { cx,
        <div style="max-width: 100%;" class="container pl-4 mt-4 pr-4">
            <div class="box">
                <figure class="image is-16by9">
                    <svg viewBox=view_box class="has-ratio">
                        <rect width="100%" height="100%" rx="10" ry="10" fill=background_color/>
                        {children(cx)}
                    </svg>
                </figure>
            </div>
        </div>
    }
}
