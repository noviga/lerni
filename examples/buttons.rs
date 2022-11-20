use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::function_component;

#[function_component(Buttons)]
pub fn buttons() -> Html {
    let onclick = Button::callback()
        .set_text(|current| format!("{}+{}", current, "Clicked"))
        .set_color(|_| Color::Honeydew)
        .set_border_color(|_| Color::ForestGreen)
        .build();

    slide(
        grid(vec![
            button("Alice").onclick(onclick.clone()),
            button("Bob").onclick(onclick.clone()),
            button("Charlie").onclick(onclick.clone()),
            button("Dave").onclick(onclick),
        ])
        .cols(2)
        .rows(2),
    )
    .into()
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Buttons>();
}
