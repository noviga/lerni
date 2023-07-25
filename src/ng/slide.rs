use leptos::{ev::resize, *};
use leptos_use::*;

use crate::ng::{provide_frame, Color, Frame, Metadata};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const SLIDE_MARGIN: i32 = 32;

#[component]
pub fn Slide(
    cx: Scope,
    #[prop(default = WIDTH)] width: i32,
    #[prop(default = HEIGHT)] height: i32,
    #[prop(optional)] background_color: Color,
    #[prop(optional)] _background_image: String,
    #[prop(optional)] _pointer: bool,
    #[prop(optional)] _blur: bool,
    #[prop(default = 15)] _blur_radius: i32,
    children: Children,
) -> impl IntoView {
    let metadata = use_context::<Metadata>(cx);
    let (slide_width, set_slide_width) = create_signal(cx, crate::ng::calc_width(SLIDE_MARGIN));
    if metadata.is_none() {
        // Standalone slide usage (not within a slideshow)
        let _ = use_event_listener(cx, window(), resize, move |_| {
            set_slide_width.set(crate::ng::calc_width(SLIDE_MARGIN));
        });
    }

    let frame = Frame {
        width,
        height,
        ..Default::default()
    };
    provide_frame(cx, frame);

    let view_box = format!("0 0 {width} {height}");

    view! { cx,
        <div
            class="container pl-4 mt-4 pr-4"
            style:max-width=move || {
                if metadata.is_none() { format!("{}px", slide_width.get()) } else { "100%".to_string() }
            }
        >
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
