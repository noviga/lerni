use leptos::*;

use crate::{use_frame, use_frames, Frame};

/// Stack of widgets.
#[component]
pub fn Stack(count: usize, children: Children) -> impl IntoView {
    let f = use_frame();
    {
        let frames = use_frames();
        let mut frames = frames.borrow_mut();
        for _ in 0..count {
            let frame = Frame {
                x: f.x,
                y: f.y,
                width: f.width,
                height: f.height,
            };
            frames.push(frame);
        }
    }

    children().nodes.into_iter().take(count).collect_view()
}
