use leptos::prelude::*;
use lerni::*;

/// Blur example.
#[component]
pub fn Blur() -> impl IntoView {
    let (blur, set_blur) = signal(false);
    let on_click = move |_| set_blur.set(!blur.get());

    view! {
        <Slide blur=blur background_color=Color::MistyRose>
            <Button on_click=on_click>
                "Blur " {move || if blur.get() { "ON" } else { "OFF" }}
            </Button>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(Blur);
}
