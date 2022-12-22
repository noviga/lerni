use lerni::{properties::*, widgets::*};
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
            <Grid cols=3 rows=3>
                <Button text="Alice" onclick={ onclick.clone() } />
                <Button text="Bob" width=300 height=300 radius=150 onclick={ onclick.clone() } />
                <Button text="Charlie" font_size=72 text_color={ Color::DarkCyan } onclick={ onclick.clone() } />
                <Button html={ html!(<><tspan font-size="96" fill="red">{ "Da" }</tspan><tspan font-size="80">{ "ve" }</tspan></>) }
                    onclick={ onclick.clone() } />
                <Label text={ format!("Clicked: {}", *counter) }/>
                <Button text="Eve" bold_text=true align={ Align::Right } onclick={ onclick.clone() } />
                <Button text="Ferdie" align={ Align::Right } valign={ VAlign::Bottom } onclick={ onclick.clone() } />
                <Button text="George" color={ Color::Honeydew } border_color={ Color::ForestGreen } onclick={ onclick.clone() } />
                <Button text="Harry" align={ Align::Fill } valign={ VAlign::Fill } { onclick } />
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start::<Buttons>();
}
