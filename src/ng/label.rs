use leptos::*;

use crate::ng::{Align, Color, Frame, VAlign};

#[component]
pub fn Label(
    cx: Scope,
    #[prop(optional)] text: &'static str,
    #[prop(optional)] _bold: bool,
    #[prop(optional)] _font: String,
    #[prop(default = 48)] font_size: i32,
    #[prop(default=Align::Center)] align: Align,
    #[prop(default=VAlign::Middle)] valign: VAlign,
    #[prop(default=Color::Black)] color: Color,
) -> impl IntoView {
    let f = use_context::<Frame>(cx).unwrap();

    let (x, anchor) = match align {
        Align::Left => (f.x, "start"),
        Align::Center | Align::Fill => (f.x + f.width / 2, "middle"),
        Align::Right => (f.x + f.width, "end"),
    };
    let (y, baseline) = match valign {
        VAlign::Top => (f.y, "hanging"),
        VAlign::Middle | VAlign::Fill => ((f.y + f.height / 2), "central"),
        VAlign::Bottom => (f.y + f.height, "text-top"),
    };

    view! { cx,
        <text x={ x.to_string() } y={ y.to_string() } { class } font-size={ font_size.to_string() } text-anchor={ anchor }
            fill={ color.to_string() } dominant-baseline={ baseline } { style } pointer-events="none">
            {
                if text.is_empty() {
                    "Empty".to_string()
                } else {
                    text.to_string()
                }
            }
        </text>
    }
}
