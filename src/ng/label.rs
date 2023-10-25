use leptos::*;

use crate::ng::{use_frame, Align, Color, VAlign};

#[component]
pub fn Label(
    #[prop(optional)] bold: bool,
    #[prop(optional)] font: String,
    #[prop(default = 48)] font_size: i32,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = Color::Black)] color: Color,
    children: Children,
) -> impl IntoView {
    let f = use_frame();

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

    view! {
        <text
            class:has-text-weight-bold=bold
            style:font-family=font
            x=x
            y=y
            font-size=font_size
            text-anchor=anchor
            fill=color
            dominant-baseline=baseline
            pointer-events="none"
            style="user-select: none; -webkit-user-select: none;"
        >
            {children()}
        </text>
    }
}
