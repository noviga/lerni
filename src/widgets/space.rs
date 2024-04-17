use leptos::{component, IntoView, View};

use crate::use_frame;

#[component]
pub fn Space() -> impl IntoView {
    // We need to pop the frame to avoid overlapping with the next widget.
    _ = use_frame();

    View::default()
}
