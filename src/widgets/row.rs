use leptos::*;

use crate::{use_frame, use_frames, Color, Frame};

/// Row of widgets.
#[component]
pub fn Row(
    #[prop(optional)] cols: Option<usize>,
    #[prop(optional)] border_width: i32,
    #[prop(default = Color::Black)] border_color: Color,
    #[prop(optional)] stretch: Vec<i32>,
    #[prop(optional)] spacing: i32,
    #[prop(optional)] padding: i32,
    children: Children,
) -> impl IntoView {
    if cols.is_none() && stretch.is_empty() {
        logging::warn!("Row: either `cols` or `stretch` must be specified");
    }

    let stretch = if let Some(cols) = cols {
        (0..cols).map(|i| *stretch.get(i).unwrap_or(&1)).collect()
    } else {
        stretch
    };
    let denominator: i32 = stretch.iter().sum();
    let cols = stretch.len();

    let f = use_frame();

    let s = spacing * (cols as i32 - 1);
    let mut x = f.x + border_width / 2;
    let y = f.y + border_width / 2;
    let height = f.height - border_width;

    let cells: Vec<_> = (0..cols)
        .map(|i| {
            let width = (f.width - border_width - s) * stretch[i] / denominator;
            let cell = Frame {
                x,
                y,
                width,
                height,
            };
            x += width + spacing;
            cell
        })
        .collect();

    {
        let frames = use_frames();
        let mut frames = frames.borrow_mut();
        for i in (0..cols).rev() {
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

    let border = (0..cols)
        .map(|i| {
            let Frame {
                x,
                y,
                width,
                height,
            } = cells[i];
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
        {border}
        {children()}
    }
}
