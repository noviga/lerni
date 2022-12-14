use lerni::widgets::*;
use wasm_bindgen::prelude::wasm_bindgen;
use yew::prelude::*;

#[function_component]
pub fn Buttons() -> Html {
    let counter = use_state(|| 0);
    let onclick = {
        let counter = counter.clone();
        Callback::from(move |_| counter.set(*counter + 1))
    };

    html! {
        <Slide>
            <Grid cols=2 rows=2>
                <Button text="Alice"   onclick={ onclick.clone() } />
                <Button text="Bob"     onclick={ onclick.clone() } />
                <Button text="Charlie" onclick={ onclick.clone() } />
                <Button html={ html!(<tspan font-size="96">{ "Dave" }</tspan>) } { onclick } />
            </Grid>
            <Label text={ format!("Clicked: {}", *counter) }/>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Buttons>();
}
