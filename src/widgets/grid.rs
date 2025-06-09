use leptos::prelude::*;

use crate::{Color, Frame, use_frame, use_frames};

/// Grid layout widget.
#[component]
pub fn Grid(
    #[prop(default = 1)] rows: usize,
    #[prop(default = 1)] cols: usize,
    #[prop(optional)] border_width: i32,
    #[prop(default = Color::Black)] border_color: Color,
    #[prop(default = Color::Transparent)] background_color: Color,
    #[prop(optional)] spacing: i32,
    #[prop(optional)] padding: i32,
    #[prop(default = true.into(), into)] visible: Signal<bool>,
    #[prop(default = "all .3s".to_string(), into)] transition: String,
    children: Children,
) -> impl IntoView {
    let f = use_frame();

    let max = cols * rows;

    let cols = cols as i32;
    let rows = rows as i32;
    let hspacing = spacing * (cols - 1);
    let vspacing = spacing * (rows - 1);
    let width = (f.width - border_width - hspacing) / cols;
    let height = (f.height - border_width - vspacing) / rows;

    {
        let frames = use_frames();
        let mut frames = frames.lock().unwrap();
        for i in (0..max).rev() {
            let x = f.x + border_width / 2 + (width + spacing) * (i as i32 % cols);
            let y = f.y + border_width / 2 + (height + spacing) * (i as i32 / cols);
            let frame = Frame {
                x: x + padding,
                y: y + padding,
                width: width - 2 * padding,
                height: height - 2 * padding,
            };
            frames.push(frame);
        }
    }

    let border = (0..max)
        .map(|i| {
            let x = f.x + border_width / 2 + (width + spacing) * (i as i32 % cols);
            let y = f.y + border_width / 2 + (height + spacing) * (i as i32 / cols);
            view! {
                <rect
                    x=x
                    y=y
                    width=width
                    height=height
                    fill="none"
                    stroke=border_color
                    stroke-width=border_width
                ></rect>
            }
        })
        .collect_view();

    view! {
        <g
            style:opacity=move || if visible.get() { "1" } else { "0" }
            style:visibility=move || { if visible.get() { "visible" } else { "hidden" } }
            style:transition=transition
        >
            <rect
                x=f.x
                y=f.y
                width=f.width
                height=f.height
                fill=background_color
                pointer-events="none"
                style="user-select: none; -webkit-user-select: none;"
            ></rect>
            {border}
            {children()}
        </g>
    }
}
