use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component(RowsCols)]
pub fn rows_cols() -> Html {
    html! {
        <Slide>
            <Row border_width=4 stretch={ vec![1, 1, 4, 1, 1] }>
                <Label text="1" />
                <Label text="2" />
                <Label text="3" />
                <Column border_width=4 stretch={ vec![1, 2, 3, 4] }>
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
