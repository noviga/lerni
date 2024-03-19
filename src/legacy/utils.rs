//! Utility functions and various helpers.

use gloo_events::EventListener;
use rand::rngs::OsRng;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use yew::{html::Scope, prelude::*};

use yew::{BaseComponent, Renderer};

/// Start function.
///
/// # Example
///
/// ```no_run
/// use yew::prelude::*;
/// use wasm_bindgen::prelude::wasm_bindgen;
///
/// #[function_component]
/// pub fn HelloWorld() -> Html {
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

/// Keyboard key codes.
pub mod keys {
    /// Backspace key.
    pub const BACKSPACE: u32 = 8;
    /// Tab key.
    pub const TAB: u32 = 9;
    /// Enter key.
    pub const ENTER: u32 = 13;
    /// Escape key.
    pub const ESCAPE: u32 = 27;
    /// Space key.
    pub const SPACE: u32 = 32;
    /// Left arrow key.
    pub const ARROW_LEFT: u32 = 37;
    /// Up arrow key.
    pub const ARROW_UP: u32 = 38;
    /// Right arrow key.
    pub const ARROW_RIGHT: u32 = 39;
    /// Down arrow key.
    pub const ARROW_DOWN: u32 = 40;
    /// Digit 0 key.
    pub const DIGIT_0: u32 = 48;
    /// Digit 1 key.
    pub const DIGIT_1: u32 = 49;
    /// Digit 2 key.
    pub const DIGIT_2: u32 = 50;
    /// Digit 3 key.
    pub const DIGIT_3: u32 = 51;
    /// Digit 4 key.
    pub const DIGIT_4: u32 = 52;
    /// Digit 5 key.
    pub const DIGIT_5: u32 = 53;
    /// Digit 6 key.
    pub const DIGIT_6: u32 = 54;
    /// Digit 7 key.
    pub const DIGIT_7: u32 = 55;
    /// Digit 8 key.
    pub const DIGIT_8: u32 = 56;
    /// Digit 9 key.
    pub const DIGIT_9: u32 = 57;
}

/// Creates Random Number Generator (RNG).
pub fn rng() -> OsRng {
    Default::default()
}

/// Add keyboard handler.
///
/// `messages` is an assotiative array with the keyboard key as a key ansd a message as a value.
pub fn add_key_handler<T, M>(link: &Scope<T>, messages: HashMap<u32, M>)
where
    T: Component,
    M: Into<T::Message> + Copy + 'static,
{
    let doc = web_sys::window().and_then(|win| win.document());
    if let Some(doc) = doc {
        let link = link.clone();
        let event = EventListener::new(&doc, "keydown", move |e| {
            if let Some(e) = e.dyn_ref::<KeyboardEvent>() {
                let key = e.key_code();
                if messages.contains_key(&key) {
                    link.send_message(messages[&key]);
                }
            }
        });
        event.forget();
    }
}

/// Add resize event handler.
pub fn add_resize_handler<T, M>(link: &Scope<T>, message: M)
where
    T: Component,
    M: Into<T::Message> + Copy + 'static,
{
    let win = web_sys::window();
    if let Some(win) = win {
        let link = link.clone();
        let event = EventListener::new(&win, "resize", move |_| {
            link.send_message(message);
        });
        event.forget();
    }
}
