use leptos::*;
use lerni::ng::*;

#[component]
pub fn RowsCols(cx: Scope) -> impl IntoView {
    view! { cx,
        <Slide>
            <Row border_width=4 stretch=vec![1, 1, 4, 1, 1] padding=20>
                <Label text=|_| "1"/>
                <Button text=|_| "2" align=Align::Fill valign=VAlign::Fill on_click=|_| ()/>
                <Label text=|_| "3" bold=true/>
                <Column border_width=4 stretch=vec![1, 2, 3, 4] spacing=20>
                    <Label text=|_| "4"/>
                    <Label text=|_| "5"/>
                    <Button text=|_| "6" align=Align::Fill valign=VAlign::Fill on_click=|_| ()/>
                    <Label text=|_| "7"/>
                </Column>
                <Label text=|_| "8"/>
            </Row>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::ng::start(RowsCols);
}
