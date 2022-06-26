use lerni::{Set, Text};
use yew::prelude::*;

#[function_component(Lesson)]
pub fn lesson() -> Html {
    html! {
        <Set>
            <Text text={ "Hello" } />
            <Text text={ "World!" } />
        </Set>
    }
}

fn main() {
    lerni::start::<Lesson>();
}
