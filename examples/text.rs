use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn TextExample() -> Html {
    html! {
        <Slide>
            <Row padding=20 border_width=4>
                <Text>
                    { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                    { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                </Text>
                <Text font_size=72 bold=true font="serif">
                    { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
                    { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                </Text>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<TextExample>();
}
