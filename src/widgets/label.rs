use leptos::*;
use web_sys::MouseEvent;

use crate::{use_frame, Align, Color, Size, VAlign};

#[component]
pub fn Label(
    #[prop(optional)] bold: bool,
    #[prop(optional)] font: String,
    #[prop(default = 48.into(), into)] font_size: Size,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = Color::Black.into(), into)] color: MaybeSignal<Color>,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
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
        <rect
            x=f.x
            y=f.y
            width=f.width
            height=f.height
            fill="transparent"
            stroke="none"
            on:click=move |e| {
                if let Some(cb) = on_click {
                    cb.call(e);
                }
            }
        >
        </rect>
        <text
            class:has-text-weight-bold=bold
            style:font-family=font
            x=x
            y=y
            font-size=move || font_size.into_pixels(f.height)
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
