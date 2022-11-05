use lerni::{
    components::Text,
    layout::{Frame, Grid, Slide},
};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(GridExample)]
pub fn grid() -> Html {
    html! {
        <Slide>
            <Frame>
                <Grid columns=3 rows=3>
                    { for (1..=9).map(|i| html_nested!(<Text text={ i.to_string() } />))}
                </Grid>
            </Frame>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<GridExample>();
}
