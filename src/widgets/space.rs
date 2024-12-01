use leptos::{component, view, IntoView};

use crate::use_frame;

/// Empty space.
#[component]
pub fn Space() -> impl IntoView {
    // We need to pop the frame to avoid overlapping with the next widget.
    _ = use_frame();

    view!()
}
