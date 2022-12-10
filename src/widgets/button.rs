use yew::prelude::*;

use crate::{
    properties::Color,
    widgets::{Frame, Label},
};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 150;

/// Button widget.
#[function_component]
pub fn Button(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let x = (f.x + (f.width - WIDTH) / 2 + props.border_width / 2).to_string();
    let y = (f.y + (f.height - HEIGHT) / 2 + props.border_width / 2).to_string();
    let width = (WIDTH - props.border_width).to_string();
    let height = (HEIGHT - props.border_width).to_string();

    let mouse_down = use_state(|| false);
    let onmousedown = {
        let mouse_down = mouse_down.clone();
        Callback::from(move |_| mouse_down.set(true))
    };
    let onmouseup = {
        let mouse_down = mouse_down.clone();
        let props = props.clone();
        Callback::from(move |_| {
            mouse_down.set(false);
            props.onclick.emit(props.clone());
        })
    };

    let border_width = if *mouse_down {
        props.border_width + 6
    } else {
        props.border_width
    };

    html! {
        <a { onmousedown } { onmouseup }>
            <rect { x } { y } { width } { height }
                rx={ props.radius.to_string() } ry={ props.radius.to_string() }
                fill={ props.color.to_string() } stroke={ props.border_color.to_string() }
                stroke-width={ border_width.to_string() } />
            <ContextProvider<Frame> context={ f }>
                <Label text={ props.text.clone() } />
            </ContextProvider<Frame>>
        </a>
    }
}

/// Button properties.
#[derive(Default, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Button".to_string())]
    pub text: String,
    #[prop_or(24)]
    pub radius: i32,
    #[prop_or(Color::AliceBlue)]
    pub color: Color,
    #[prop_or(12)]
    pub border_width: i32,
    #[prop_or(Color::RoyalBlue)]
    pub border_color: Color,
    #[prop_or_default]
    pub onclick: Callback<Props>,
}
