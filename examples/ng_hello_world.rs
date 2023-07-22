use leptos::*;
use lerni::ng::*;

#[component]
pub fn HelloWorld(cx: Scope) -> impl IntoView {
    view! { cx,
        <SlideShow>
            <Slide background_color={ Color::MistyRose }><Label text="Hello →".into()/></Slide>
            <Slide background_color={ Color::PaleGreen }><Label text="← World!".into()/></Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(HelloWorld);
}
