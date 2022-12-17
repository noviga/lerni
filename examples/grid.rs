use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(GridExample)]
pub fn grid_example() -> Html {
    html! {
        <Slide>
            <Grid cols=3 rows=3>
                <Label html={ html!(<tspan font-size="96">{ "1" }</tspan>) } />
                <Label text="2" />
                <Label html={ html!(<tspan fill="red">{ "3" }</tspan>) } />
                <Label text="4" />
                <Label text="5" />
                <Label text="6" />
                <Label text="7" />
                <Label text="8" />
                <Grid cols=4 rows=2 border_width=6>
                    <Label text="9"  />
                    <Label text="10" />
                    <Label text="11" />
                    <Label text="12" />
                    <Label text="13" />
                    <Label text="14" />
                    <Label text="15" />
                    <Label text="16" />
                </Grid>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<GridExample>();
}
