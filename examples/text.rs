use leptos::{ev::keydown, prelude::*};
use leptos_use::use_event_listener;
use lerni::*;

/// Text example.
#[component]
pub fn TextExample() -> impl IntoView {
    let word_count1 = RwSignal::new(0);
    let words_read1 = RwSignal::new(0);
    let letters_read1 = RwSignal::new(0);
    let letters_total1 = RwSignal::new(0);

    let words_read2 = RwSignal::new(0);
    let letters_read2 = RwSignal::new(0);
    let letters_total2 = RwSignal::new(0);

    let words_read3 = RwSignal::new(0);
    let letters_read3 = RwSignal::new(0);
    let letters_total3 = RwSignal::new(0);

    let (lattice, set_lattice) = signal(true);
    let (reverse, set_reverse) = signal(true);
    let (shuffle, set_shuffle) = signal(true);
    let (erase, set_erase) = signal(0.3);

    let node_ref = NodeRef::new();
    _ = use_event_listener(document().body(), keydown, move |e| {
        if is_active_slide(node_ref) {
            if e.key() == " " {
                if words_read1.get() < word_count1.get() {
                    words_read1.update(|wr| *wr += 1);
                }
            } else if e.key() == "Escape" {
                words_read1.update(|wr| *wr = 0);
            }
        }
    });

    view! {
        <Slide node_ref=node_ref>
            <Row cols=2 padding=30 border_width=4>
                <Column stretch=[5, 1, 5, 1]>
                    <Text
                        lattice=lattice
                        word_count=word_count1
                        words_read=words_read1
                        letters_read=letters_read1
                        letters_total=letters_total1
                    >
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}
                        {"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}
                    </Text>
                    <Label>{words_read1} "w (" {letters_read1} " / " {letters_total1} ")"</Label>
                    <Text valign=VAlign::Bottom reverse_words=reverse>
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}
                        {"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}
                    </Text>
                    <Row cols=2 spacing=10>
                        <Button
                            align=Align::Fill
                            valign=VAlign::Fill
                            on_click=move |_| set_lattice.set(!lattice.get())
                        >
                            "Lattice"
                        </Button>
                        <Button
                            align=Align::Fill
                            valign=VAlign::Fill
                            on_click=move |_| set_reverse.set(!reverse.get())
                        >
                            "Reverse"
                        </Button>
                    </Row>
                </Column>

                <Column stretch=[5, 1, 5, 1, 1]>
                    <Text
                        font_size=48
                        bold=true
                        font="serif"
                        valign=VAlign::Middle
                        shuffle_letters=shuffle
                        words_read=words_read2
                        letters_read=letters_read2
                        letters_total=letters_total2
                    >
                        {"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."}
                        {"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </Text>
                    <Label>{words_read2} "w (" {letters_read2} " / " {letters_total2} ")"</Label>
                    <Text
                        font_size=48
                        erase_top=erase
                        valign=VAlign::Bottom
                        words_read=words_read3
                        letters_read=letters_read3
                        letters_total=letters_total3
                    >
                        {"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."}
                        {"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </Text>
                    <Label>{words_read3} "w (" {letters_read3} " / " {letters_total3} ")"</Label>
                    <Row cols=2 spacing=10>
                        <Button
                            align=Align::Fill
                            valign=VAlign::Fill
                            on_click=move |_| set_shuffle.set(!shuffle.get())
                        >
                            "Shuffle"
                        </Button>
                        <Button
                            align=Align::Fill
                            valign=VAlign::Fill
                            on_click=move |_| set_erase.set(0.3 - erase.get())
                        >
                            "Erase"
                        </Button>
                    </Row>
                </Column>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(TextExample);
}
