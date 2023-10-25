use leptos::*;
use lerni::ng::*;

#[component]
pub fn TextExample(cx: Scope) -> impl IntoView {
    let (words_read1, _) = create_signal(cx, 0);

    let (words_read2, _) = create_signal(cx, 0);

    let (words_read3, _) = create_signal(cx, 0);

    // let words_read1 = use_state(|| 0);
    // let read1 = use_state(|| "".to_string());
    // let onread1 = {
    //     let words_read1 = words_read1.clone();
    //     let read1 = read1.clone();
    //     Callback::from(move |(words_read, letters, total)| {
    //         words_read1.set(words_read);
    //         read1.set(format!("{letters} / {total}"));
    //     })
    // };

    // let words_read2 = use_state(|| 0);
    // let read2 = use_state(|| "".to_string());
    // let onread2 = {
    //     let words_read2 = words_read2.clone();
    //     let read2 = read2.clone();
    //     Callback::from(move |(words_read, letters, total)| {
    //         words_read2.set(words_read);
    //         read2.set(format!("{letters} / {total}"));
    //     })
    // };

    // let words_read3 = use_state(|| 0);
    // let read3 = use_state(|| "".to_string());
    // let onread3 = {
    //     let words_read3 = words_read3.clone();
    //     let read3 = read3.clone();
    //     Callback::from(move |(words_read, letters, total)| {
    //         words_read3.set(words_read);
    //         read3.set(format!("{letters} / {total}"));
    //     })
    // };

    view! { cx,
        <Slide>
            <Row padding=30 border_width=4>
                <Column stretch={ vec![5, 1] }>
                    <Text>// lattice=true words_read={ *words_read1 } onread={ onread1 }>
                        { "Lorem ipsum dolor sit amet, consectetur adipiscing elit, sed do eiusmod tempor incididunt ut labore et dolore magna aliqua." }
                        { "Ut enim ad minim veniam, quis nostrud exercitation ullamco laboris nisi ut aliquip ex ea commodo consequat." }
                    </Text>
                    <Label>{words_read1}</Label>
                </Column>

                <Column stretch={ vec![5, 1, 5, 1] }>
                    <Text>// font_size=48 bold=true font="serif" words_read={ *words_read2 } onread={ onread2 }>
                        { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
                        { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                    </Text>
                    <Label>{words_read2}</Label>
                    <Text>// font_size=48 erase_top=0.3 words_read={ *words_read3 } onread={ onread3 }>
                        { "Duis aute irure dolor in reprehenderit in voluptate velit esse cillum dolore eu fugiat nulla pariatur." }
                        { "Excepteur sint occaecat cupidatat non proident, sunt in culpa qui officia deserunt mollit anim id est laborum." }
                    </Text>
                    <Label>{words_read3}</Label>
                </Column>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(TextExample);
}
