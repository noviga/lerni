use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(RowsCols)]
pub fn raws_cols() -> Html {
    html! {
        <Slide>
            <Row>
                <Label text="1" />
                <Label text="2" />
                <Label text="3" />
                <Column>
                    <Label text="4" />
                    <Label text="5" />
                    <Label text="6" />
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
