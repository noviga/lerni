use leptos::*;
use lerni::*;

#[component]
pub fn Pointer() -> impl IntoView {
    let (pointer, set_pointer) = create_signal(true);
    let on_click = move |_| set_pointer.set(!pointer.get());

    view! {
        <Slide pointer=pointer.into()>
            <Button on_click=on_click>
                "Pointer " {move || if pointer.get() { "ON" } else { "OFF" }}
            </Button>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(Pointer);
}
