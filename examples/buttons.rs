use lerni::{
    components::Button,
    layout::{Grid, Slide},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(Buttons)]
pub fn buttons() -> Html {
    let onclick = Button::callback()
        .set_text(|current| format!("{}+{}", current, "Clicked"))
        .build();

    html! {
        <Slide>
            <Grid cols=2 rows=2>
                <Button text="Alice" onclick={ onclick.clone() } />
                <Button text="Bob" onclick={ onclick.clone() } />
                <Button text="Charlie" onclick={ onclick.clone() } />
                <Button text="Dave" { onclick } />
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Buttons>();
}
