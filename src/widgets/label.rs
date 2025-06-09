use leptos::prelude::*;

use crate::{Align, Color, Flip, Size, VAlign, use_frame};

#[component]
pub fn Label(
    #[prop(optional)] bold: bool,
    #[prop(default = "sans-serif".to_string(), into)] font: String,
    #[prop(default = 48.into(), into)] font_size: Size,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = Color::Black.into(), into)] color: Signal<Color>,
    #[prop(default = Color::Transparent.into(), into)] background_color: Signal<Color>,
    #[prop(default = true.into(), into)] visible: Signal<bool>,
    #[prop(default = "all .3s".to_string(), into)] transition: String,
    #[prop(optional)] angle: i32,
    #[prop(optional)] flip: Flip,
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

    let mut transform = format!("rotate({angle} {x} {y})");
    if flip == Flip::Horizontal {
        transform.push_str(" scale(-1 1)");
    } else if flip == Flip::Vertical {
        transform.push_str(" scale(1 -1)");
    }

    view! {
        <g
            style:opacity=move || if visible.get() { "1" } else { "0" }
            style:visibility=move || { if visible.get() { "visible" } else { "hidden" } }
            style:transition=transition
        >
            <rect
                x=f.x
                y=f.y
                width=f.width
                height=f.height
                fill=background_color
                pointer-events="none"
                style="user-select: none; -webkit-user-select: none;"
            ></rect>
            <text
                class:has-text-weight-bold=bold
                style:font-family=font
                x=if flip == Flip::Horizontal { -x } else { x }
                y=if flip == Flip::Vertical { -y } else { y }
                transform=transform
                font-size=move || font_size.into_pixels(f.height)
                text-anchor=anchor
                fill=color
                dominant-baseline=baseline
                pointer-events="none"
                style="user-select: none; -webkit-user-select: none;"
            >
                {children()}
            </text>
        </g>
    }
}
