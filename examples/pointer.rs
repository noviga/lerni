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

    html! {
        <Slide pointer={ *pointer }>
            <Button text="Pointer ON/OFF" { onclick } />
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Pointer>();
}
