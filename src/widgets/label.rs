use yew::prelude::*;

use crate::{
    properties::{Align, VAlign},
    widgets::Frame,
};

/// Label widget.
#[function_component]
pub fn Label(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let anchor = match props.align {
        Align::Left => "start",
        Align::Center => "middle",
        Align::Right => "end",
    };
    let baseline = match props.valign {
        VAlign::Top => "hanging",
        VAlign::Middle => "central",
        VAlign::Bottom => "text-top",
    };
    let x = (f.x + f.width / 2).to_string();
    let y = (f.y + f.height / 2).to_string();

    html! {
        <text { x } { y } font-size={ props.font_size.to_string() } text-anchor={ anchor }
            dominant-baseline={ baseline } pointer-events="none">
            { &props.text }
        </text>
    }
}

/// Label properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Label".to_string())]
    pub text: String,
    /// Font size (default: 48px).
    #[prop_or(48)]
    pub font_size: usize,
    /// Horizontal align (default: Center).
    #[prop_or(Align::Center)]
    pub align: Align,
    /// Vertical align (default: Middle).
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
}
