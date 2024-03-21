//! Widgets are the main building blocks.

mod button;
mod column;
mod grid;
mod label;
mod row;
mod slide;
mod slideshow;
mod stack;
mod svg;
mod text;

pub use button::Button;
pub use column::Column;
pub use grid::Grid;
pub use label::Label;
pub use row::Row;
pub use slide::Slide;
pub use slideshow::SlideShow;
pub use stack::Stack;
pub use svg::{Svg, SvgFile};
pub use text::Text;

/// Additional information provided to all slides.
#[derive(Clone, Copy, Default, Debug, PartialEq)]
pub struct Metadata {
    /// Teacher mode flag.
    pub teacher_mode: bool,
    /// Pointer on/off flag.
    pub pointer: bool,
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
