use leptos::*;
use lerni::*;

#[component]
pub fn StackExample() -> impl IntoView {
    view! {
        <Slide>
            <Stack count=2>
                <SvgFile width=128 height=64 scale=2.0 src=include_str!("logo.svg") />
                <Label font_size=96 valign=VAlign::Top>"Hello,"</Label>
            </Stack>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(StackExample);
}
