use lerni::{
    components::Text,
    layout::{Slide, SlideShow},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <SlideShow>
            <Slide background="#FFCCCC">
                <Text text="Hello →" />
            </Slide>
            <Slide background="#CCFFCC">
                <Text text="← World!" />
            </Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
