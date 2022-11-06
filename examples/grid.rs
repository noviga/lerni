use lerni::{
    components::Text,
    layout::{Grid, Slide},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(GridExample)]
pub fn grid() -> Html {
    html! {
        <Slide>
            <Grid cols=3 rows=3>
                { for (1..=9).map(|i| html_nested!(<Text>{ i.to_string() }</Text>))}
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<GridExample>();
}
