use lerni::{Set, Text};
use yew::prelude::*;
use wasm_bindgen::prelude::*;

#[function_component(Lesson)]
pub fn lesson() -> Html {
    html! {
        <Set>
            <Text text={ "Hello" } />
            <Text text={ "World!" } />
        </Set>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Lesson>();
}
