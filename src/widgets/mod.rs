//! Widgets are the main building blocks.

mod button;
mod common;
mod grid;
mod label;
mod slide;
mod slideshow;

pub use button::{button, Button};
pub use common::{MultiWidget, Widget};
pub use grid::{grid, Grid};
pub use label::{label, Label};
pub use slide::{slide, Slide};
pub use slideshow::{slideshow, SlideShow};
