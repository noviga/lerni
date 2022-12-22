//! Widgets are the main building blocks.

mod button;
mod column;
mod grid;
mod label;
mod row;
mod slide;
mod slideshow;

pub use button::Button;
pub use column::Column;
pub use grid::Grid;
pub use label::Label;
pub use row::Row;
pub use slide::Slide;
pub use slideshow::SlideShow;

/// Additional information provided to all slides.
#[derive(Clone, Default, PartialEq)]
pub struct Metadata {
    /// Visibility flag.
    pub visible: bool,
    /// Teacher mode flag.
    pub teacher_mode: bool,
    /// Pointer on/off flag.
    pub pointer: bool,
}

/// Frame within which the widget will be rendered.
#[derive(Clone, Default, PartialEq)]
pub struct Frame {
    /// X-coordinate (in pixels) of the to left corner.
    pub x: i32,
    /// Y-coordinate (in pixels) of the to left corner.
    pub y: i32,
    /// Width (in pixels).
    pub width: i32,
    /// Height (in pixels).
    pub height: i32,
}
