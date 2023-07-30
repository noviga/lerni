use leptos::*;
use lerni::ng::*;

#[component]
pub fn Buttons(cx: Scope) -> impl IntoView {
    let (counter, set_counter) = create_signal(cx, 0);
    let on_click = move |_| {
        log!("Clicked");
        set_counter.set(counter.get() + 1);
    };

    view! { cx,
        <Slide>
            <Grid cols=3 rows=3>
                <Button text=|_| "Alice" on_click=on_click/>
                <Button text=|_| "Bob" width=300 height=300 radius=150 on_click=on_click/>
                <Button text=|_| "Charlie" font_size=72 text_color=Color::DarkCyan on_click=on_click/>
                <Button text=move |_| view! { cx,
                    <tspan font-size="96" fill="red" alignment-baseline="central">"Da"</tspan>
                    <tspan font-size="80" alignment-baseline="central">"ve"</tspan>
                } on_click=on_click/>
                <Label text=move |_| counter.get()/>
                <Button text=|_| "Eve" text_bold=true align=Align::Right on_click=on_click/>
                <Button text=|_| "Ferdie" align=Align::Right valign=VAlign::Bottom on_click=on_click/>
                <Button text=|_| "George" color=Color::Honeydew border_color=Color::ForestGreen on_click=on_click/>
                <Button text=|_| "Harry" align=Align::Fill valign=VAlign::Fill on_click=on_click/>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(Buttons);
}
