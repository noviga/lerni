use yew::prelude::*;

use crate::{
    properties::{Align, Color, VAlign},
    widgets::{Frame, Label},
};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 150;

/// Button properties.
#[derive(Default, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub text_bold: bool,
    #[prop_or_default]
    pub html: Html,
    #[prop_or(WIDTH)]
    pub width: i32,
    #[prop_or(HEIGHT)]
    pub height: i32,
    #[prop_or(24)]
    pub radius: i32,
    #[prop_or_default]
    pub font: String,
    #[prop_or(48)]
    pub font_size: i32,
    #[prop_or(Color::AliceBlue)]
    pub color: Color,
    #[prop_or(Color::Black)]
    pub text_color: Color,
    #[prop_or(12)]
    pub border_width: i32,
    #[prop_or(Color::RoyalBlue)]
    pub border_color: Color,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
    #[prop_or_default]
    pub onclick: Callback<Props>,
}

/// Button widget.
#[function_component]
pub fn Button(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let width = if props.align == Align::Fill {
        f.width
    } else {
        props.width
    };
    let height = if props.valign == VAlign::Fill {
        f.height
    } else {
        props.height
    };

    let x = match props.align {
        Align::Left | Align::Fill => f.x,
        Align::Center => f.x + (f.width - width) / 2,
        Align::Right => f.x + f.width - width,
    };
    let y = match props.valign {
        VAlign::Top | VAlign::Fill => f.y,
        VAlign::Middle => f.y + (f.height - height) / 2,
        VAlign::Bottom => f.y + f.height - height,
    };

    let mouse_down = use_state(|| false);
    let onmousedown = {
        let mouse_down = mouse_down.clone();
        Callback::from(move |_| mouse_down.set(true))
    };
    let onmouseup = {
        let mouse_down = mouse_down.clone();
        let props = props.clone();
        Callback::from(move |_| {
            if *mouse_down {
                mouse_down.set(false);
                props.onclick.emit(props.clone());
            }
        })
    };
    let onmouseleave = {
        let mouse_down = mouse_down.clone();
        Callback::from(move |_| {
            mouse_down.set(false);
        })
    };

    let border_width = if *mouse_down {
        props.border_width + 6
    } else {
        props.border_width
    };

    let frame = Frame {
        x,
        y,
        width,
        height,
        ..f
    };
    let x = (x + border_width / 2).to_string();
    let y = (y + border_width / 2).to_string();
    let width = (width - border_width).to_string();
    let height = (height - border_width).to_string();
    html! {
        <a { onmousedown } { onmouseup } { onmouseleave }>
            <rect { x } { y } { width } { height }
                rx={ props.radius.to_string() } ry={ props.radius.to_string() }
                fill={ props.color.to_string() } stroke={ props.border_color.to_string() }
                stroke-width={ border_width.to_string() } />
            <ContextProvider<Frame> context={ frame }>
                <Label text={ props.text.clone() } html={ props.html.clone() } bold={ props.text_bold }
                    color={ props.text_color } font={ props.font.clone() } font_size={ props.font_size }/>
            </ContextProvider<Frame>>
        </a>
    }
}
