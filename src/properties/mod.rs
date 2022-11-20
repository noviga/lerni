//! Components properties.

mod align;
mod color;

/// Main SVG width in pixels.
pub const SVG_WIDTH: usize = 640;
/// Main SVG height in pixels.
pub const SVG_HEIGHT: usize = 360;

pub use align::{Align, VAlign};
pub use color::Color;
