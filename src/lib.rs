//! Lerni library.

#![warn(missing_docs)]

pub mod properties;
pub mod widgets;

use yew::Component;

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
pub fn start<T: Component>()
where
    <T as yew::Component>::Properties: Default,
{
    yew::start_app::<T>();
}
