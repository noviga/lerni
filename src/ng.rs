//! New generation Lerni based on Leptos.
//!
pub use wasm_bindgen::prelude::wasm_bindgen;

mod align;
pub use align::{Align, VAlign};

mod color;
pub use color::Color;

mod label;
pub use label::Label;

mod slide;
pub use slide::Slide;

mod slideshow;
pub use slideshow::SlideShow;

use leptos::{IntoView, Scope};

/// Additional information provided to all slides.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Metadata {
    /// Visibility flag.
    pub visible: bool,
    /// Teacher mode flag.
    pub teacher_mode: bool,
    /// Pointer on/off flag.
    pub pointer: bool,
}

/// Frame within which the widget will be rendered.
#[derive(Clone, Default, Debug, PartialEq)]
pub struct Frame {
    /// X-coordinate (in pixels) of the to left corner.
    pub x: i32,
    /// Y-coordinate (in pixels) of the to left corner.
    pub y: i32,
    /// Width (in pixels).
    pub width: i32,
    /// Height (in pixels).
    pub height: i32,
    /// Screen X to SVG X transform factor.
    pub fx: f32,
    /// Screen Y to SVG Y transform factor.
    pub fy: f32,
}

/// The main entry point.
pub fn start<F, N>(f: F)
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    leptos::mount_to_body(f);
}
