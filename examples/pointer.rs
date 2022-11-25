use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(Pointer)]
pub fn pointer() -> Html {
    let pointer = use_state(|| true);
    let onclick = {
        let pointer = pointer.clone();
        Callback::from(move |_| pointer.set(!*pointer))
    };
    slide(button("Pointer ON/OFF").onclick(onclick))
        .pointer(*pointer)
        .into()
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Pointer>();
}
