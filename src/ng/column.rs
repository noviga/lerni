use leptos::*;

use crate::ng::{use_frame, use_frames, Color, Frame};

/// Column of widgets.
#[component]
pub fn Column(
    cx: Scope,
    #[prop(optional)] rows: Option<usize>,
    #[prop(optional)] border_width: i32,
    #[prop(default = Color::Black)] border_color: Color,
    #[prop(optional)] stretch: Vec<i32>,
    #[prop(optional)] spacing: i32,
    #[prop(optional)] padding: i32,
    children: Children,
) -> impl IntoView {
    if rows.is_none() && stretch.is_empty() {
        warn!("Column: either `rows` or `stretch` must be specified");
    }

    let stretch = if let Some(rows) = rows {
        (0..rows).map(|i| *stretch.get(i).unwrap_or(&1)).collect()
    } else {
        stretch
    };
    let denominator: i32 = stretch.iter().sum();
    let rows = stretch.len();

    let f = use_frame(cx);

    let s = spacing * (rows as i32 - 1);
    let x = f.x + border_width / 2;
    let mut y = f.y + border_width / 2;
    let width = f.width - border_width;

    let cells: Vec<_> = (0..rows)
        .map(|i| {
            let height = (f.height - border_width - s) * stretch[i] / denominator;
            let cell = Frame {
                x,
                y,
                width,
                height,
            };
            y += height + spacing;
            cell
        })
        .collect();

    {
        let frames = use_frames(cx);
        let mut frames = frames.borrow_mut();
        for i in (0..rows).rev() {
            let Frame {
                x,
                y,
                width,
                height,
            } = cells[i];
            let frame = Frame {
                x: x + padding,
                y: y + padding,
                width: width - 2 * padding,
                height: height - 2 * padding,
            };
            frames.push(frame);
        }
    }

    children(cx).nodes.into_iter().take(rows).enumerate().map(|(i, child)| {
        let Frame {x, y, width, height } = cells[i];
        view! { cx,
            <rect x=x y=y width=width height=height fill="none" stroke=border_color stroke-width=border_width/>
            {child}
        }
    }).collect_view(cx)
}
