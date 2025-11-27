//! Widgets are the main building blocks.

mod button;
mod column;
mod grid;
mod image;
mod label;
mod row;
mod slide;
mod slideshow;
mod space;
mod stack;
mod svg;
mod text;

pub use button::Button;
pub use column::Column;
pub use grid::Grid;
pub use image::Image;
pub use label::Label;
pub use row::Row;
pub use slide::Slide;
pub use slideshow::{SlideSet, SlideShow};
pub use space::Space;
pub use stack::Stack;
pub use svg::{Svg, SvgFile};
pub use text::{Strings, Text};

use leptos::prelude::ReadSignal;

/// Size in pixels or percent.
#[derive(Clone, Copy, Debug)]
pub enum Size {
    /// Size in pixels.
    Pixels(i32),
    /// Size in percent.
    Percent(i32),
}

impl From<i32> for Size {
    fn from(value: i32) -> Self {
        Size::Pixels(value)
    }
}

impl Size {
    /// Converts the size to pixels.
    pub fn into_pixels(self, total: i32) -> i32 {
        match self {
            Size::Pixels(value) => value,
            Size::Percent(value) => total * value / 100,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct PointerSignal(ReadSignal<bool>);

impl PointerSignal {
    pub fn new(signal: ReadSignal<bool>) -> Self {
        PointerSignal(signal)
    }
}

#[derive(Clone, Copy, Debug)]
pub(crate) struct RefreshSignal(ReadSignal<()>);

impl RefreshSignal {
    pub fn new(signal: ReadSignal<()>) -> Self {
        RefreshSignal(signal)
    }
}

/// Calculates the width of the slide.
pub fn calc_width(x_margin: i32, y_margin: i32) -> i32 {
    let elem = web_sys::window()
        .and_then(|win| win.document())
        .and_then(|doc| doc.document_element());
    if let Some(elem) = elem {
        let width = elem.client_width() - x_margin;
        let height = elem.client_height();
        width.min((height - y_margin) * 16 / 9)
    } else {
        0
    }
}
