use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn TextExample() -> Html {
    let x = use_state(|| 0);
    let y = use_state(|| 0);
    let onclick = {
        let x = x.clone();
        let y = y.clone();
        Callback::from(move |(mx, my)| {
            x.set(mx);
            y.set(my);
        })
    };

    let read1 = use_state(|| "".to_string());
    let onread1 = {
        let read1 = read1.clone();
        Callback::from(move |(letters, total)| {
            read1.set(format!("{letters} / {total}"));
        })
    };

    let read2 = use_state(|| "".to_string());
    let onread2 = {
        let read2 = read2.clone();
        Callback::from(move |(letters, total)| {
            read2.set(format!("{letters} / {total}"));
        })
    };

    html! {
        <Slide { onclick }>
            <Row padding=30 border_width=4>
                <Column stretch={ vec![5, 1] }>
                    <Text marker_x={ *x } marker_y={ *y } onread={ onread1 }>
                        { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                        { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                    </Text>
                    <Label text={ (*read1).clone() } />
                </Column>
                <Column stretch={ vec![5, 1] }>
                    <Text font_size=72 bold=true font="serif" marker_x={ *x } marker_y={ *y } onread={ onread2 }>
                        { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
                        { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                    </Text>
                    <Label text={ (*read2).clone() } />
                </Column>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<TextExample>();
}