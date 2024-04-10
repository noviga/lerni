use leptos::{ev::keydown, *};
use leptos_use::use_event_listener;
use lerni::*;

#[component]
pub fn TextExample() -> impl IntoView {
    let word_count1 = create_rw_signal(0);
    let words_read1 = create_rw_signal(0);
    let letters_read1 = create_rw_signal(0);
    let letters_total1 = create_rw_signal(0);

    let words_read2 = create_rw_signal(0);
    let letters_read2 = create_rw_signal(0);
    let letters_total2 = create_rw_signal(0);

    let words_read3 = create_rw_signal(0);
    let letters_read3 = create_rw_signal(0);
    let letters_total3 = create_rw_signal(0);

    let node_ref = create_node_ref();
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
                <Column stretch=vec![5, 1]>
                    <Text
                        lattice=true
                        word_count=word_count1
                        words_read=words_read1
                        letters_read=letters_read1
                        letters_total=letters_total1
                    >
                        {"Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua."}
                        {"Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat."}
                    </Text>
                    <Label>{words_read1} "w (" {letters_read1} " / " {letters_total1} ")"</Label>
                </Column>

                <Column stretch=vec![5, 1, 5, 1]>
                    <Text
                        font_size=48
                        bold=true
                        font="serif"
                        valign=VAlign::Middle
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
                        erase_top=0.3
                        valign=VAlign::Bottom
                        words_read=words_read3
                        letters_read=letters_read3
                        letters_total=letters_total3
                    >
                        {"Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur."}
                        {"Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum."}
                    </Text>
                    <Label>{words_read3} "w (" {letters_read3} " / " {letters_total3} ")"</Label>
                </Column>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(TextExample);
}
