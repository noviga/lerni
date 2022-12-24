use yew::prelude::*;

use crate::{
    properties::{Align, Color, VAlign},
    widgets::Frame,
};

/// Label properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub text: String,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub html: Html,
    /// Font size (default: 48px).
    #[prop_or(48)]
    pub font_size: usize,
    /// Horizontal align (default: Center).
    #[prop_or(Align::Center)]
    pub align: Align,
    /// Vertical align (default: Middle).
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
    #[prop_or(Color::Black)]
    pub color: Color,
}

/// Label widget.
#[function_component]
pub fn Label(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let anchor = match props.align {
        Align::Left => "start",
        Align::Center | Align::Fill => "middle",
        Align::Right => "end",
    };
    let baseline = match props.valign {
        VAlign::Top => "hanging",
        VAlign::Middle | VAlign::Fill => "central",
        VAlign::Bottom => "text-top",
    };
    let x = (f.x + f.width / 2).to_string();
    let y = (f.y + f.height / 2).to_string();

    let class = classes!(props.bold.then_some("has-text-weight-bold"));

    html! {
        <text { x } { y } { class } font-size={ props.font_size.to_string() } text-anchor={ anchor }
            fill={ props.color.to_string() } dominant-baseline={ baseline } pointer-events="none">
            {
                if props.text.is_empty() {
                    props.html.clone()
                } else {
                    props.text.clone().into()
                }
            }
        </text>
    }
}
