use lerni::{
    components::Text,
    layout::{Frame, SlideShow},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(HelloWorld)]
pub fn hello_world() -> Html {
    html! {
        <SlideShow>
            <Frame background="#FFCCCC">
                <Text text="Hello →" />
            </Frame>
            <Frame background="#CCFFCC">
                <Text text="← World!" />
            </Frame>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<HelloWorld>();
}
