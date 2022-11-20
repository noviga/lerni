use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::function_component;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    slideshow(vec![
        slide(label("Hello →")).background(Color::MistyRose),
        slide(label("← World!")).background(Color::PaleGreen),
    ])
    .into()
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
