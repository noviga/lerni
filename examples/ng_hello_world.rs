use leptos::*;
use lerni::ng::*;

#[component]
pub fn HelloWorld(cx: Scope) -> impl IntoView {
    view! { cx,
        <SlideShow>
            <Slide background_color={ Color::MistyRose }><Label text="Hello →"  /></Slide>
            <Slide background_color={ Color::PaleGreen }><Label text="← World!" /></Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(HelloWorld);
}
