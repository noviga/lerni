use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <SlideShow>
            <Slide background={ Color::MistyRose }><Label text="Hello →"  /></Slide>
            <Slide background={ Color::PaleGreen }><Label text="← World!" /></Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
