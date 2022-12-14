//! Lerni library.

#![warn(missing_docs)]

pub mod properties;
pub mod utils;
pub mod widgets;

use yew::{BaseComponent, Renderer};

/// Start function.
///
/// # Example
///
/// ```no_run
/// use yew::prelude::*;
/// use wasm_bindgen::prelude::wasm_bindgen;
///
/// #[function_component(HelloWorld)]
/// pub fn hello_world() -> Html {
///     html!("Hello, world!")
/// }
///
/// #[wasm_bindgen(start)]
/// pub fn main() {
///     lerni::start::<HelloWorld>();
/// }
/// ```
pub fn start<T: BaseComponent>()
where
    <T as BaseComponent>::Properties: Default,
{
    Renderer::<T>::new().render();
}

/// Debug macro.
#[macro_export]
macro_rules! debug {
    ($arg:literal) => {
        web_sys::console::log_1(&format!("{}", $arg).into())
    };
    ($arg:expr) => {
        web_sys::console::log_1(&format!("{:?}", $arg).into())
    };
    ($fmt:literal $(, $args:expr)+) => {
        web_sys::console::log_1(&format!($fmt $(, $args)+).into())
    };
}
