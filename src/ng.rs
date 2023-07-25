//! New generation Lerni based on Leptos.

pub use wasm_bindgen::prelude::wasm_bindgen;

mod align;
pub use align::{Align, VAlign};

mod color;
pub use color::Color;

mod frame;
pub use frame::*;

mod grid;
pub use grid::Grid;

mod label;
pub use label::Label;

mod slide;
pub use slide::Slide;

mod slideshow;
pub use slideshow::SlideShow;

use leptos::*;

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

/// Calculates the width of the slide.
pub fn calc_width(margin: i32) -> i32 {
    let elem = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.document_element());
    if let Some(elem) = elem {
        let width = elem.client_width();
        let height = elem.client_height();
        width.min((height - margin) * 16 / 9)
    } else {
        0
    }
}

/// The main entry point.
pub fn start<F, N>(f: F)
where
    F: FnOnce(Scope) -> N + 'static,
    N: IntoView,
{
    leptos::mount_to_body(f);
}
