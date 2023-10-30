use leptos::*;
use lerni::*;

#[component]
pub fn RowsCols() -> impl IntoView {
    view! {
        <Slide>
            <Row border_width=4 stretch=vec![1, 1, 4, 1, 1] padding=20>
                <Label>"1"</Label>
                <Button align=Align::Fill valign=VAlign::Fill on_click=|_| ()>"2"</Button>
                <Label bold=true>"3"</Label>
                <Column border_width=4 stretch=vec![1, 2, 3, 4] spacing=20>
                    <Label>"4"</Label>
                    <Label>"5"</Label>
                    <Button align=Align::Fill valign=VAlign::Fill on_click=|_| ()>"6"</Button>
                    <Label>"7"</Label>
                </Column>
                <Label>"8"</Label>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(RowsCols);
}
