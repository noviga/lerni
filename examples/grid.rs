use leptos::*;
use lerni::*;

#[component]
pub fn GridExample() -> impl IntoView {
    view! {
        <Slide>
            <Grid cols=3 rows=3 border_width=4 padding=20>
                <Label>
                    <tspan>"00"</tspan>
                    <tspan font-size=96>"1"</tspan>
                </Label>
                <Label>2</Label>
                <Label>
                    <tspan>"00"</tspan>
                    <tspan fill="red">"3"</tspan>
                </Label>
                <Label font_size=96 color=Color::Blue>
                    "4"
                </Label>
                <Button
                    align=Align::Fill
                    valign=VAlign::Fill
                    on_click=move |_| logging::log!("5: Clicked")
                >
                    "5"
                </Button>
                <Label align=Align::Right>"6"</Label>
                <Label align=Align::Left>"7"</Label>
                <Image src="/img/lerni-bg.svg"/>
                <Grid cols=4 rows=2 border_width=4 spacing=20>
                    <Label valign=VAlign::Top>"9"</Label>
                    <Label valign=VAlign::Bottom>"10"</Label>
                    <Label>11</Label>
                    <Button
                        align=Align::Fill
                        valign=VAlign::Fill
                        on_click=move |_| logging::log!("12: Clicked")
                    >
                        "12"
                    </Button>
                    <Label>"13"</Label>
                    <Label align=Align::Left valign=VAlign::Top>
                        "14"
                    </Label>
                    <Label>"15"</Label>
                    <Label align=Align::Right valign=VAlign::Bottom>
                        "16"
                    </Label>
                </Grid>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(GridExample);
}
