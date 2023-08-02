use leptos::*;
use lerni::ng::*;

#[component]
pub fn HelloWorld(cx: Scope) -> impl IntoView {
    view! { cx,
        <SlideShow>
            <Slide background_color=Color::MistyRose>
                <Label>"Hello →"</Label>
            </Slide>
            <Slide background_color=Color::PaleGreen>
                <Label>"← World!"</Label>
            </Slide>
        </SlideShow>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(HelloWorld);
}
