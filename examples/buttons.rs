use leptos::*;
use lerni::*;

/// Buttons example.
#[component]
pub fn Buttons() -> impl IntoView {
    let (counter, set_counter) = create_signal(0);
    let on_click = move |_| {
        logging::log!("Clicked");
        set_counter.set(counter.get() + 1);
    };
    let bob_color = Signal::derive(move || {
        if counter.get() % 2 == 0 {
            Color::LightCoral
        } else {
            Color::LightSkyBlue
        }
    });

    view! {
        <Slide>
            <Grid cols=3 rows=3>
                <Button on_click=on_click visible=Signal::derive(move || counter.get() % 2 == 0)>
                    "Alice"
                </Button>
                <Button width=300 rounded=true color=bob_color on_click=on_click>
                    "Bob"
                </Button>
                <Button font_size=72 text_color=Color::DarkCyan on_click=on_click>
                    "Charlie"
                </Button>
                <Button on_click=on_click>
                    <tspan font-size="96" fill="red" alignment-baseline="central">
                        "Da"
                    </tspan>
                    <tspan font-size="80" alignment-baseline="central">
                        "ve"
                    </tspan>
                </Button>
                <Label font_size=Size::Percent(50)>{counter}</Label>
                <Button text_bold=true align=Align::Right on_click=on_click>
                    "Eve"
                </Button>
                <Button
                    width=Size::Percent(50)
                    align=Align::Right
                    valign=VAlign::Bottom
                    on_click=on_click
                >
                    "Ferdie"
                </Button>
                <Button color=Color::Honeydew border_color=Color::ForestGreen on_click=on_click>
                    "George"
                </Button>
                <Button
                    align=Align::Fill
                    valign=VAlign::Fill
                    on_click=on_click
                    image="/img/lerni-bg.svg"
                    margin=20
                >
                    "Harry"
                </Button>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(Buttons);
}
