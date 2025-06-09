use leptos::{
    ev::{MouseEvent, resize},
    html::Div,
    prelude::*,
    svg::Svg,
};
use leptos_use::*;
use std::{
    collections::VecDeque,
    sync::{Arc, Mutex},
};

use crate::{Color, Frame, PointerSignal, RefreshSignal, SvgFrame, is_active_slide, provide_frame};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;
const POINTER_SIZE: i32 = 72;
const SLIDE_MARGIN: i32 = 32;

#[component]
pub fn Slide(
    #[prop(default = WIDTH)] width: i32,
    #[prop(default = HEIGHT)] height: i32,
    #[prop(optional)] background_color: Color,
    #[prop(optional, into)] background_image: String,
    #[prop(optional, into)] blur: Signal<bool>,
    #[prop(default = 15)] blur_radius: i32,
    #[prop(optional)] node_ref: Option<NodeRef<Div>>,
    #[prop(optional, into)] on_click: Option<Callback<(i32, i32)>>,
    #[prop(optional, into)] on_refresh: Option<Callback<()>>,
    #[prop(optional)] panel: Option<AnyView>,
    children: Children,
) -> impl IntoView {
    let panels: Option<Arc<Mutex<VecDeque<AnyView>>>> = use_context();
    if let Some(panels) = panels {
        let mut panels = panels.lock().unwrap();
        panels.push_back(panel.unwrap_or_else(|| view!().into_any()));
    }

    let refresh_signal = use_context::<RefreshSignal>();
    let pointer_signal = use_context::<PointerSignal>();
    let (slide_width, set_slide_width) = signal(crate::calc_width(0, SLIDE_MARGIN));

    // Standalone slide usage (not within a slideshow)
    let is_standalone = pointer_signal.is_none();

    if is_standalone {
        _ = use_event_listener(window(), resize, move |_| {
            set_slide_width.set(crate::calc_width(0, SLIDE_MARGIN));
        });
    }

    let frame = Frame {
        width,
        height,
        ..Default::default()
    };
    provide_frame(frame);

    let view_box = format!("0 0 {width} {height}");

    let svg_ref = NodeRef::<Svg>::new();
    let (pointer_position, set_pointer_position) = signal((0, 0));
    let on_mousemove = move |e: MouseEvent| {
        let mut px = e.offset_x();
        let mut py = e.offset_y();
        if let Some(svg) = svg_ref.get() {
            px = px * WIDTH / svg.client_width();
            py = py * HEIGHT / svg.client_height();
            provide_context(SvgFrame {
                width: WIDTH,
                height: HEIGHT,
                client_width: svg.client_width(),
                client_height: svg.client_height(),
            });
        }
        set_pointer_position.set((px, py));
    };
    let on_mouse_click = move |e: MouseEvent| {
        if let Some(f) = &on_click {
            if let Some(svg) = svg_ref.get() {
                let x = e.offset_x() * WIDTH / svg.client_width();
                let y = e.offset_y() * HEIGHT / svg.client_height();
                f.run((x, y));
            }
        }
    };
    let (pointer_in, set_pointer_in) = signal(false);
    let pointer_visible =
        move || pointer_signal.map(|s| s.0.get()).unwrap_or_default() && pointer_in.get();

    let blur_style = move || {
        let radius = if blur.get() { blur_radius } else { 0 };
        format!(
            r#"-webkit-filter: blur({radius}px);
            -moz-filter: blur({radius}px);
            -ms-filter: blur({radius}px);
            filter: blur({radius}px); transition: all .3s;"#
        )
    };

    let bg_style = if !background_image.is_empty() {
        format!(
            r#"background-image: url({background_image});
            background-size: cover;
            background-position: center;
            background-repeat: no-repeat;"#
        )
    } else {
        Default::default()
    };

    let node_ref = node_ref.unwrap_or_else(NodeRef::new);

    Effect::new(move |_| {
        if let Some(refresh) = refresh_signal {
            refresh.0.get();
        }
        if is_active_slide(node_ref) {
            if let Some(cb) = on_refresh {
                cb.run(())
            }
        }
    });

    view! {
        <div
            node_ref=node_ref
            class="container pl-4 mt-4 pr-4"
            style:max-width=move || {
                if is_standalone { format!("{}px", slide_width.get()) } else { "100%".to_string() }
            }
        >

            <div class="box">
                <figure class="image is-16by9" style=blur_style>
                    <svg
                        on:mousemove=on_mousemove
                        on:mouseenter=move |_| set_pointer_in.set(true)
                        on:mouseleave=move |_| set_pointer_in.set(false)
                        on:click=on_mouse_click
                        node_ref=svg_ref
                        viewBox=view_box
                        class="has-ratio"
                        style=bg_style
                    >
                        <rect
                            width="100%"
                            height="100%"
                            rx="10"
                            ry="10"
                            fill=background_color
                        ></rect>
                        {children()}

                        <Pointer position=pointer_position visible=pointer_visible />
                    </svg>
                </figure>
            </div>
        </div>
    }
}

#[component]
fn Pointer<F>(position: ReadSignal<(i32, i32)>, visible: F) -> impl IntoView
where
    F: Fn() -> bool + Send + 'static,
{
    view! {
        <circle
            cx=move || position.get().0
            cy=move || position.get().1
            style="filter: blur(2px); transition: opacity .3s;"
            r=POINTER_SIZE / 2
            fill="orange"
            opacity=move || if visible() { 0.75 } else { 0.0 }
            pointer-events="none"
        ></circle>
    }
}
