use leptos::{ev::keydown, html::Div, *};
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
        if is_slide_visible(&node_ref) {
            if e.key() == "Enter" {
                set_counter.set(counter.get() + 1);
            } else if e.key() == "Escape" && counter.get() > 0 {
                set_counter.set(counter.get() - 1);
            }
        }
    });

    let on_click = move |_| {
        let panel_item_refs: Option<Vec<NodeRef<Div>>> = use_context();
        if let Some(panel_item_refs) = panel_item_refs {
            let slide_number = slide_number(&node_ref);
            if let Some(n) = slide_number {
                let panel_item_ref = panel_item_refs.get(n);
                if let Some(panel_item_ref) = panel_item_ref {
                    if let Some(e) = panel_item_ref.get() {
                        _ = e.inner_html("<label class=\"label is-large\">Counter: 0</label>");
                    }
                }
            }
        }
    };

    view! {
        <Slide node_ref=node_ref>
            <Column rows=2>
                <Label>
                    "Counter (press 'Enter' and 'Escape' to change): " {move || counter.get()}
                </Label>
                <Button on_click=on_click>
                    "Push Me"
                </Button>
            </Column>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(HelloWorld);
}
