use lerni::{
    properties::{Align, VAlign},
    widgets::*,
};
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn RowsCols() -> Html {
    html! {
        <Slide>
            <Row border_width=4 stretch={ vec![1, 1, 4, 1, 1] } padding=20>
                <Label text="1" />
                <Button text="2" align={ Align::Fill } valign={ VAlign::Fill } />
                <Label text="3" bold=true />
                <Column border_width=4 stretch={ vec![1, 2, 3, 4] } spacing=20>
                    <Label text="4" />
                    <Label text="5" />
                    <Button text="6" align={ Align::Fill } valign={ VAlign::Fill } />
                    <Label text="7" />
                </Column>
                <Label text="8" />
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<RowsCols>();
}
