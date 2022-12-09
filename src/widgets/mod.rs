//! Widgets are the main building blocks.

mod button;
mod column;
mod common;
mod grid;
mod label;
mod label2;
mod row;
mod slide;
mod slide2;
mod slideshow;

pub use button::{button, Button};
pub use column::Column;
pub use common::{MultiWidget, Widget, Widget2, WidgetObject};
pub use grid::{grid, Grid};
pub use label::{label, Label};
pub use label2::Label2;
pub use row::Row;
pub use slide::{slide, Slide};
pub use slide2::Slide2;
pub use slideshow::{slideshow, SlideShow};
