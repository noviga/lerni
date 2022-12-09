use lerni::{properties::Color, widgets::*};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(Buttons)]
pub fn buttons() -> Html {
    let onclick = Button::callback()
        .set_text(|current| format!("{}+{}", current, "Clicked"))
        .set_color(|_| Color::Honeydew)
        .set_border_color(|_| Color::ForestGreen)
        .build();

    html! {
        <Slide>
            <Grid cols=2 rows=2>
                <Button text="Alice"   onclick={ onclick.clone() } />
                <Button text="Bob"     onclick={ onclick.clone() } />
                <Button text="Charlie" onclick={ onclick.clone() }/ >
                <Button text="Dave"  { onclick }/ >
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Buttons>();
}
