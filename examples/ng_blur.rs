use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn Blur() -> Html {
    let blur = use_state(|| false);
    let onclick = {
        let blur = blur.clone();
        Callback::from(move |_| blur.set(!*blur))
    };

    html! {
        <Slide blur={ *blur } background_color={ Color::MistyRose }>
            <Button text="Blur ON/OFF" { onclick } />
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Blur>();
}
