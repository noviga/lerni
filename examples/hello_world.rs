use leptos::{ev::keydown, *};
use leptos_use::use_event_listener;
use lerni::*;

#[component]
pub fn HelloWorld() -> impl IntoView {
    view! {
        <SlideShow>
            <Slide background_color=Color::MistyRose>
                <Label>"Hello →"</Label>
            </Slide>
            <Slide background_color=Color::PaleGreen>
                <Label>"← World!"</Label>
            </Slide>
            <Counter/>
        </SlideShow>
    }
}

#[component]
pub fn Counter() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);

    let node_ref = create_node_ref();
    _ = use_event_listener(document().body(), keydown, move |e| {
        if is_active_slide(node_ref) {
            if e.key() == "Enter" {
                set_counter.set(counter.get() + 1);
            } else if e.key() == "Escape" && counter.get() > 0 {
                set_counter.set(counter.get() - 1);
            }
        }
    });

    create_effect(move |_| {
        if let Some(el) = node_ref.get() {
            let panel_item =
                view! { <label class="label is-large">"Counter: " {move || counter.get()}</label> };
            _ = el.on_mount(move |_| mount_on_panel(node_ref, panel_item));
        }
    });

    let on_click = move |(x, y)| logging::log!("({}, {})", x, y);

    view! {
        <Slide node_ref=node_ref on_click=on_click on_refresh=move |_| set_counter.set(0)>
            <Label>
                "Counter (press 'Enter' and 'Escape' to change): " {move || counter.get()}
            </Label>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(HelloWorld);
}
