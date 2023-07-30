use leptos::*;

use crate::ng::{use_frame, use_frames, Color, Frame};

/// Grid layout widget.
#[component]
pub fn Grid(
    cx: Scope,
    #[prop(default = 1)] rows: usize,
    #[prop(default = 1)] cols: usize,
    #[prop(optional)] border_width: i32,
    #[prop(default = Color::Black)] border_color: Color,
    #[prop(optional)] spacing: i32,
    #[prop(optional)] padding: i32,
    children: Children,
) -> impl IntoView {
    let f = use_frame(cx);

    let max = cols * rows;

    let cols = cols as i32;
    let rows = rows as i32;
    let hspacing = spacing * (cols - 1);
    let vspacing = spacing * (rows - 1);
    let width = (f.width - border_width - hspacing) / cols;
    let height = (f.height - border_width - vspacing) / rows;

    {
        let frames = use_frames(cx);
        let mut frames = frames.borrow_mut();
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

    children(cx).nodes.into_iter().take(max).enumerate().map(|(i, child)| {
        let x = f.x + border_width / 2 + (width + spacing)  * (i as i32 % cols);
        let y = f.y + border_width / 2 + (height + spacing) * (i as i32 / cols);
        view! { cx,
            <rect x=x y=y width=width height=height fill="none" stroke=border_color stroke-width=border_width/>
            {child}
        }
    }).collect_view(cx)
}
