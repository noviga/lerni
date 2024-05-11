use leptos::*;
use lerni::*;

/// SVG example.
#[component]
pub fn SvgExample() -> impl IntoView {
    view! {
        <Slide background_image="/img/lerni-bg.svg">
            <Grid cols=3 rows=3 border_width=4 padding=20>
                <SvgFile width=128 height=64 src=include_str!("logo.svg")/>
                <Svg width=128 height=64 scale=3.0>
                    {include!("logo.svg-rs")}
                </Svg>
                <Svg width=128 height=64 scale=2.0 align=Align::Left valign=VAlign::Top>
                    {include!("logo.svg-rs")}
                </Svg>
                <SvgFile width=128 height=64 align=Align::Fill src=include_str!("logo.svg")/>
                <Svg width=128 height=64 flip_x=true>
                    {include!("logo.svg-rs")}
                </Svg>
                <Svg width=128 height=64 scale=2.0>
                    {include!("logo.svg-rs")}
                </Svg>
                <Svg width=128 height=64 valign=VAlign::Bottom>
                    {include!("logo.svg-rs")}
                </Svg>
                <Svg width=128 height=64 flip_y=true>
                    {include!("logo.svg-rs")}
                </Svg>
                <Svg
                    width=128
                    height=64
                    flip_x=true
                    flip_y=true
                    scale=2.0
                    align=Align::Right
                    valign=VAlign::Bottom
                >
                    {include!("logo.svg-rs")}
                </Svg>
            </Grid>
        </Slide>
    }
}

#[wasm_bindgen(start)]
pub fn main() {
    lerni::start(SvgExample);
}
