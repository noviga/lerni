use leptos::*;
use web_sys::MouseEvent;

use crate::ng::{provide_frame, use_frame, Align, Color, Frame, Label, VAlign};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 150;

#[component]
pub fn Button<CB>(
    cx: Scope,
    on_click: CB,
    #[prop(optional)] text_bold: bool,
    #[prop(default = WIDTH)] width: i32,
    #[prop(default = HEIGHT)] height: i32,
    #[prop(default = 24)] radius: i32,
    #[prop(optional)] font: String,
    #[prop(default = 48)] font_size: i32,
    #[prop(default = Color::AliceBlue)] color: Color,
    #[prop(default = Color::Black)] text_color: Color,
    #[prop(default = 12)] border_width: i32,
    #[prop(default = Color::RoyalBlue)] border_color: Color,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    children: Children,
) -> impl IntoView
where
    CB: FnMut(MouseEvent) + 'static,
{
    let f = use_frame(cx);

    let width = if align == Align::Fill { f.width } else { width };
    let height = if valign == VAlign::Fill {
        f.height
    } else {
        height
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
        x,
        y,
        width,
        height,
    };
    provide_frame(cx, frame);

    let x = x + border_width / 2;
    let y = y + border_width / 2;
    let width = width - border_width;
    let height = height - border_width;

    let (border, set_border) = create_signal(cx, border_width);
    let on_mousedown = move |_| set_border.set(border_width + 6);
    let on_mouseup = move |_| set_border.set(border_width);

    view! { cx,
        <rect
            on:click=on_click
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
        <Label bold=text_bold color=text_color font=font font_size=font_size>
            {children(cx)}
        </Label>
    }
}
