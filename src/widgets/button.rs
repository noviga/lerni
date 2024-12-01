use leptos::prelude::*;
use web_sys::MouseEvent;

use crate::{provide_frame, use_frame, Align, Color, Frame, Label, Size, VAlign};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 150;

#[component]
pub fn Button<F>(
    #[prop(optional)] on_click: Option<F>,
    #[prop(optional, into)] on_mousedown: Option<Callback<MouseEvent>>,
    #[prop(optional, into)] on_mouseup: Option<Callback<MouseEvent>>,
    #[prop(optional)] text_bold: bool,
    #[prop(optional, into)] width: Option<Size>,
    #[prop(optional, into)] height: Option<Size>,
    #[prop(optional, into)] radius: Option<i32>,
    #[prop(default = false)] rounded: bool,
    #[prop(optional, into)] font: String,
    #[prop(default = 48.into(), into)] font_size: Size,
    #[prop(default = Color::AliceBlue.into(), into)] color: Signal<Color>,
    #[prop(default = Color::Black.into(), into)] text_color: Signal<Color>,
    #[prop(default = 12.into(), into)] border_width: Signal<i32>,
    #[prop(default = Color::RoyalBlue.into(), into)] border_color: Signal<Color>,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(optional)] margin: i32,
    #[prop(optional, into)] image: Signal<String>,
    #[prop(default = true.into(), into)] visible: Signal<bool>,
    #[prop(default = "all .3s".to_string(), into)] transition: String,
    children: Children,
) -> impl IntoView
where
    F: Fn(MouseEvent) + 'static,
{
    let f = use_frame();

    let width = width.map(|s| s.into_pixels(f.width));
    let height = height.map(|s| s.into_pixels(f.height));

    let size = if rounded {
        width.or(height).or(radius.map(|r| r * 2))
    } else {
        None
    };

    let (width, height, radius) = if rounded {
        let size = size.unwrap_or(WIDTH);
        (size, size, size / 2)
    } else {
        let width = if align == Align::Fill {
            f.width
        } else {
            width.unwrap_or(WIDTH)
        };
        let height = if valign == VAlign::Fill {
            f.height
        } else {
            height.unwrap_or(HEIGHT)
        };
        (width, height, radius.unwrap_or(24))
    };

    let x = match align {
        Align::Left | Align::Fill => f.x,
        Align::Center => f.x + (f.width - width) / 2,
        Align::Right => f.x + f.width - width,
    };
    let y = match valign {
        VAlign::Top | VAlign::Fill => f.y,
        VAlign::Middle => f.y + (f.height - height) / 2,
        VAlign::Bottom => f.y + f.height - height,
    };

    let frame = Frame {
        x: x + margin,
        y: y + margin,
        width: width - 2 * margin,
        height: height - 2 * margin,
    };
    provide_frame(frame);

    let x = x + border_width.get() / 2;
    let y = y + border_width.get() / 2;
    let width = width - border_width.get();
    let height = height - border_width.get();

    let (border, set_border) = signal(border_width.get());
    let on_mousedown = move |e| {
        set_border.set(border_width.get() + 6);
        if let Some(cb) = on_mousedown {
            cb.run(e);
        }
    };
    let on_mouseup = move |e| {
        set_border.set(border_width.get());
        if let Some(cb) = on_mouseup {
            cb.run(e);
        }
    };

    view! {
        <g
            style:opacity=move || if visible.get() { "1" } else { "0" }
            style:visibility=move || { if visible.get() { "visible" } else { "hidden" } }
            style:transition=transition
        >
            <rect
                on:click=move |e| {
                    if let Some(cb) = &on_click {
                        cb(e);
                    }
                }

                on:mousedown=on_mousedown
                on:mouseup=on_mouseup
                on:mouseleave=on_mouseup
                x=x
                y=y
                width=width
                height=height
                rx=radius
                ry=radius
                fill=color
                stroke=border_color
                stroke-width=move || border.get()
                style="cursor: pointer;"
            ></rect>
            <image
                stroke=border_color
                stroke-width=4
                href=image
                x=x + margin
                y=y + margin
                width=width - 2 * margin
                height=height - 2 * margin
                preserveAspectRatio="xMidYMid meet"
                pointer-events="none"
                style="user-select: none; -webkit-user-select: none;"
            ></image>
            <Label bold=text_bold color=text_color font=font font_size=font_size>
                {children()}
            </Label>
        </g>
    }
}
