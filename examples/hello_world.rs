use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    slideshow(vec![
        slide(label("Hello →")).background("#FFCCCC"),
        slide(label("← World!")).background("#CCFFCC"),
    ])
    .into()
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
