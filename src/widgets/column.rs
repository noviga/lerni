use yew::prelude::*;

use crate::{properties::Color, widgets::Frame};

/// Column of widgets.
#[function_component]
pub fn Column(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let stretch: Vec<_> = (0..props.children.len())
        .map(|i| *props.stretch.get(i).unwrap_or(&1))
        .collect();
    let denominator: i32 = stretch.iter().sum();

    let x = f.x + props.border_width / 2;
    let mut y = f.y + props.border_width / 2;
    let width = f.width - props.border_width;

    html! {
        for props.children.iter().enumerate().map(|(i, item)| {
            let height = (f.height - props.border_width) * stretch[i] / denominator;
            let frame = Frame {
                x,
                y,
                width,
                height,
            };
            let html = html_nested! {
                <ContextProvider<Frame> context={ frame }>
                    <rect x={ x.to_string() } y={ y.to_string() } width={ width.to_string() } height={ height.to_string() }
                        fill="none" stroke={ props.border_color.to_string() } stroke-width={ props.border_width.to_string() } />
                    { item }
                </ContextProvider<Frame>>
            };
            y += height;
            html
        })
    }
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(0)]
    pub border_width: i32,
    #[prop_or(Color::Black)]
    pub border_color: Color,
    #[prop_or_default]
    pub stretch: Vec<i32>,
}
