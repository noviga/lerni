use leptos::{
    ev::{resize, MouseEvent},
    svg::Svg,
    *,
};
use leptos_use::*;

use crate::ng::{provide_frame, Color, Frame, Metadata};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const POINTER_SIZE: i32 = 72;
const SLIDE_MARGIN: i32 = 32;

#[component]
pub fn Slide(
    cx: Scope,
    #[prop(default = WIDTH)] width: i32,
    #[prop(default = HEIGHT)] height: i32,
    #[prop(optional)] background_color: Color,
    #[prop(optional)] _background_image: String,
    #[prop(optional)] pointer: MaybeSignal<bool>,
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

    let svg_ref: NodeRef<Svg> = create_node_ref(cx);
    let (pointer_position, set_pointer_position) = create_signal(cx, (0, 0));
    let on_mousemove = move |e: MouseEvent| {
        let mut px = e.offset_x();
        let mut py = e.offset_y();
        if let Some(svg) = svg_ref.get() {
            px = px * WIDTH / svg.client_width();
            py = py * HEIGHT / svg.client_height();
        }
        set_pointer_position.set((px, py));
    };
    let (pointer_in, set_pointer_in) = create_signal(cx, false);
    let pointer_visible = move || pointer.get() && pointer_in.get();

    view! { cx,
        <div
            class="container pl-4 mt-4 pr-4"
            style:max-width=move || {
                if metadata.is_none() { format!("{}px", slide_width.get()) } else { "100%".to_string() }
            }
        >
            <div class="box">
                <figure class="image is-16by9">
                    <svg
                        on:mousemove=on_mousemove
                        on:mouseenter=move |_| set_pointer_in.set(true)
                        on:mouseleave=move |_| set_pointer_in.set(false)
                        node_ref=svg_ref
                        viewBox=view_box
                        class="has-ratio"
                    >
                        <rect width="100%" height="100%" rx="10" ry="10" fill=background_color/>
                        {children(cx)}
                        <Pointer position=pointer_position visible=pointer_visible/>
                    </svg>
                </figure>
            </div>
        </div>
    }
}

#[component]
fn Pointer<F>(cx: Scope, position: ReadSignal<(i32, i32)>, visible: F) -> impl IntoView
where
    F: Fn() -> bool + 'static,
{
    view! { cx,
        <circle
            cx=move || position.get().0
            cy=move || position.get().1
            style="filter: blur(2px); transition: opacity .3s;"
            r=POINTER_SIZE / 2
            fill="orange"
            opacity=move || if visible() { 0.75 } else { 0.0 }
            pointer-events="none"
        />
    }
}
