use leptos::*;
use lerni::ng::*;
use wasm_bindgen::prelude::wasm_bindgen;

#[component]
pub fn GridExample(cx: Scope) -> impl IntoView {
    view! { cx,
        <Slide>
            <Grid cols=3 rows=3 border_width=4 padding=20>
                <Label text=move |_| view! { cx, <><tspan>"00"</tspan><tspan font-size=96>"1"</tspan></> }/>
                <Label text=|_| "2" />
                <Label text=move |_| view! { cx, <tspan>"00"</tspan><tspan fill="red">"3"</tspan> }/>
                <Label text=|_| "4" font_size=96 color=Color::Blue/>
                // <Button text=|_| "5" align=Align::Fill valign=VAlign::Fill/>
                <Label text=|_| "5" align=Align::Fill valign=VAlign::Fill/>
                <Label text=|_| "6" align=Align::Right/>
                <Label text=|_| "7" align=Align::Left/>
                <Label text=|_| "8"/>
                <Grid cols=4 rows=2 border_width=4 spacing=20>
                    <Label text=|_| "9" valign=VAlign::Top/>
                    <Label text=|_| "10" valign=VAlign::Bottom/>
                    <Label text=|_| "11" />
                    // <Button text=|_| "12" align=Align::Fill valign=VAlign::Fill/>
                    <Label text=|_| "12" align=Align::Fill valign=VAlign::Fill/>
                    <Label text=|_| "13" />
                    <Label text=|_| "14" align=Align::Left valign=VAlign::Top/>
                    <Label text=|_| "15" />
                    <Label text=|_| "16" align=Align::Right valign=VAlign::Bottom/>
                </Grid>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(GridExample);
}
