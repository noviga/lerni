//! Lerni library.

#![warn(missing_docs)]

#[cfg(feature = "legacy")]
mod legacy;
#[cfg(feature = "legacy")]
pub use legacy::*;

#[cfg(not(feature = "legacy"))]
pub use frame::*;
pub use into_strings::IntoStrings;
pub use properties::*;
pub use utils::*;
pub use widgets::*;

pub use wasm_bindgen::prelude::wasm_bindgen;

#[cfg(not(feature = "legacy"))]
mod frame;
#[cfg(not(feature = "legacy"))]
mod properties;
#[cfg(not(feature = "legacy"))]
mod utils;
#[cfg(not(feature = "legacy"))]
mod widgets;

mod into_strings;
