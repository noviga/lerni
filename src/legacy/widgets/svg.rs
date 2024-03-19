use yew::prelude::*;

use crate::{
    properties::{Align, VAlign},
    widgets::Frame,
};

/// SVG properties.
#[derive(Default, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    pub width: i32,
    pub height: i32,
    #[prop_or(Align::Center)]
    pub align: Align,
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
    #[prop_or(1.0)]
    pub scale: f32,
    #[prop_or_default]
    pub flip_x: bool,
    #[prop_or_default]
    pub flip_y: bool,
}

/// SVG widget.
#[function_component]
pub fn Svg(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let scale = if matches!(props.align, Align::Fill) || matches!(props.valign, VAlign::Fill) {
        let sx = f.width as f32 / props.width as f32;
        let sy = f.height as f32 / props.height as f32;
        sx.min(sy)
    } else {
        props.scale
    };

    let width = (scale * props.width as f32).round() as i32;
    let height = (scale * props.height as f32).round() as i32;

    let mut x = match props.align {
        Align::Left => f.x,
        Align::Center | Align::Fill => f.x + (f.width - width) / 2,
        Align::Right => f.x + f.width - width,
    };
    let mut y = match props.valign {
        VAlign::Top => f.y,
        VAlign::Middle | VAlign::Fill => f.y + (f.height - height) / 2,
        VAlign::Bottom => f.y + f.height - height,
    };

    let mut sx = scale;
    let mut sy = scale;

    if props.flip_x {
        sx = -sx;
        x += width;
    }
    if props.flip_y {
        sy = -sy;
        y += height;
    }

    html! {
        <g transform={ format!("translate({x} {y}) scale({sx} {sy})") }>
        { for props.children.iter() }
        </g>
    }
}
