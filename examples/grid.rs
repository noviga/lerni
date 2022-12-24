use lerni::{
    properties::{Align, Color, VAlign},
    widgets::*,
};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn GridExample() -> Html {
    html! {
        <Slide>
            <Grid cols=3 rows=3 border_width=4 padding=20>
                <Label html={ html!(<><tspan>{ "00" }</tspan><tspan font-size="96">{ "1" }</tspan></>) } />
                <Label text="2" />
                <Label html={ html!(<><tspan>{ "00" }</tspan><tspan fill="red">{ "3" }</tspan></>) } />
                <Label text="4" font_size=96 color={ Color::Blue } />
                <Button text="5" align={ Align::Fill } valign={ VAlign::Fill } />
                <Label text="6" />
                <Label text="7" />
                <Label text="8" />
                <Grid cols=4 rows=2 border_width=4 spacing=20>
                    <Label text="9"  />
                    <Label text="10" />
                    <Label text="11" />
                    <Button text="12" align={ Align::Fill } valign={ VAlign::Fill } />
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
