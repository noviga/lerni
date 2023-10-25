use leptos::*;

use crate::ng::{use_frame, use_frames, Color, Frame};

/// Text widget.
#[component]
pub fn Text(
    cx: Scope,
    #[prop(optional)] bold: bool,
    #[prop(default = 48)] font_size: i32,
    #[prop(default = Color::Black)] color: Color,
    #[prop(default = "sans-serif".to_string(), into)] font: String,
    #[prop(default = 1.2)] line_height: f32,
    #[prop(default = 1.4)] indent: f32,
    #[prop(default = Color::PaleGreen)] marker_color: Color,
    #[prop(optional)] words_read: usize,
    #[prop(optional)] lattice: bool,
    #[prop(optional)] erase_top: f32,
    #[prop(optional)] erase_bottom: f32,
    children: Children,
) -> impl IntoView {
    // view! { cx,
    //     {children(cx)}
    // }
    children(cx)
}
