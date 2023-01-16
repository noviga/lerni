use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn HelloWorld() -> Html {
    html! {
        <SlideShow>
            <Slide background_color={ Color::MistyRose }><Label text="Hello →"  /></Slide>
            <Slide background_color={ Color::PaleGreen }><Label text="← World!" /></Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
