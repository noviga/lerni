use lerni::widgets::*;
use wasm_bindgen::prelude::*;
use yew::prelude::*;

#[function_component(GridExample)]
pub fn grid() -> Html {
    slide(
        grid(vec![
            label("1"),
            label("2"),
            label("3"),
            label("4"),
            label("5"),
            label("6"),
            label("7"),
            label("8"),
        ])
        .cols(3)
        .rows(3)
        .add(
            grid(vec![
                label("9"),
                label("10"),
                label("11"),
                label("12"),
                label("13"),
                label("14"),
                label("15"),
                label("16"),
            ])
            .cols(4)
            .rows(2),
        ),
    )
    .into()
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<GridExample>();
}
