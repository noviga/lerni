use yew::prelude::*;

use crate::layout::{Align, VAlign};

/// Text properties.
#[derive(Properties, PartialEq)]
pub struct Props {
    /// Inner tag contents.
    pub children: Children,
    /// Font size (default: 28px).
    #[prop_or(48)]
    pub size: u32,
    /// Horizontal align (default: Center).
    #[prop_or(Align::Center)]
    pub align: Align,
    /// Vertical align (default: Middle).
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
}

/// Text component.
#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    let x = match props.align {
        Align::Left => "0",
        Align::Center => "50%",
        Align::Right => "100%",
    };
    let y = match props.valign {
        VAlign::Top => "0",
        VAlign::Middle => "50%",
        VAlign::Bottom => "100%",
    };
    let anchor = match props.align {
        Align::Left => "start",
        Align::Center => "middle",
        Align::Right => "end",
    };
    let baseline = match props.valign {
        VAlign::Top => "hanging",
        VAlign::Middle => "middle",
        VAlign::Bottom => "text-top",
    };

    html! {
        <text { x } { y } font-size={ props.size.to_string() } text-anchor={ anchor } dominant-baseline={ baseline }>
            { for props.children.iter() }
        </text>
    }
}
