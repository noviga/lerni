use lerni::{
    components::Text,
    layout::{Slide, SlideShow},
};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <SlideShow>
            <Slide bgcolor="#FFCCCC">
                <Text>{ "Hello →" }</Text>
            </Slide>
            <Slide bgcolor="#CCFFCC">
                <Text>{ "← World!" }</Text>
            </Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
