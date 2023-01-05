//! Utility functions and various helpers.

use gloo_events::EventListener;
use rand::rngs::OsRng;
use std::collections::HashMap;
use wasm_bindgen::JsCast;
use yew::{html::Scope, prelude::*};

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
