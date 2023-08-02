use leptos::*;
use lerni::ng::*;

#[component]
pub fn Blur(cx: Scope) -> impl IntoView {
    let (blur, set_blur) = create_signal(cx, false);
    let on_click = move |_| set_blur.set(!blur.get());

    view! { cx,
        <Slide blur=blur.into() background_color=Color::MistyRose>
            <Button on_click=on_click>
                "Blur " {move || if blur.get() { "ON" } else { "OFF" }}
            </Button>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(Blur);
}
