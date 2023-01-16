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
    #[prop_or_default]
    pub font: String,
    #[prop_or(48)]
    pub font_size: i32,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
    #[prop_or(Color::Black)]
    pub color: Color,
}

/// Label widget.
#[function_component]
pub fn Label(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let (x, anchor) = match props.align {
        Align::Left => (f.x, "start"),
        Align::Center | Align::Fill => (f.x + f.width / 2, "middle"),
        Align::Right => (f.x + f.width, "end"),
    };
    let (y, baseline) = match props.valign {
        VAlign::Top => (f.y, "hanging"),
        VAlign::Middle | VAlign::Fill => ((f.y + f.height / 2), "central"),
        VAlign::Bottom => (f.y + f.height, "text-top"),
    };

    let class = classes!(props.bold.then_some("has-text-weight-bold"));
    let style = classes!((!props.font.is_empty()).then(|| format!("font-family: {};", props.font)));

    html! {
        <text x={ x.to_string() } y={ y.to_string() } { class } font-size={ props.font_size.to_string() } text-anchor={ anchor }
            fill={ props.color.to_string() } dominant-baseline={ baseline } { style } pointer-events="none">
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
