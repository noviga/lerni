use lerni::{Cards, Text};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Lesson)]
pub fn lesson() -> Html {
    html! {
        <Cards>
            <Text text={ "Hello" } />
            <Text text={ "World!" } />
        </Cards>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Lesson>();
}
