use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(RowsCols)]
pub fn raws_cols() -> Html {
    html! {
        <Slide2>
            <Row>
                <Label2 text="1" />
                <Label2 text="2" />
                <Label2 text="3" />
                <Column>
                    <Label2 text="4" />
                    <Label2 text="5" />
                    <Label2 text="6" />
                </Column>
            </Row>
        </Slide2>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<RowsCols>();
}
