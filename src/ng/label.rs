use leptos::*;

use crate::ng::{Align, Color, Frame, VAlign};

#[component]
pub fn Label<F, IV>(
    cx: Scope,
    text: F,
    #[prop(optional)] frame: Option<Frame>,
    #[prop(optional)] _bold: bool,
    #[prop(optional)] _font: String,
    #[prop(default = 48)] font_size: i32,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = Color::Black)] color: Color,
) -> impl IntoView
where
    F: Fn(Scope) -> IV + 'static,
    IV: IntoView,
{
    let f = frame.or_else(|| use_context::<Frame>(cx)).unwrap();

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
        <text
            x=x
            y=y
            font-size=font_size
            text-anchor=anchor
            fill=color
            dominant-baseline=baseline
            pointer-events="none"
        >
            {text(cx)}
        </text>
    }
}
