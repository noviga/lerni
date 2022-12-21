use yew::prelude::*;

use crate::{properties::Color, widgets::Frame};

/// Grid layout widget.
#[function_component]
pub fn Grid(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let cols = props.cols as i32;
    let rows = props.rows as i32;
    let hspacing = props.spacing * (cols - 1);
    let vspacing = props.spacing * (rows - 1);
    let width = (f.width - props.border_width - hspacing) / cols;
    let height = (f.height - props.border_width - vspacing) / rows;

    let max = props.cols * props.rows;
    html! {
        for props.children.iter().take(max).enumerate().map(|(i, item)| {
            let x = f.x + props.border_width / 2 + (width + props.spacing)  * (i as i32 % cols);
            let y = f.y + props.border_width / 2 + (height + props.spacing) * (i as i32 / cols);
            let frame = Frame {
                x: x + props.padding,
                y: y + props.padding,
                width: width - 2 * props.padding,
                height: height - 2 * props.padding,
            };
            html_nested! {
                <ContextProvider<Frame> context={ frame }>
                    <rect x={ x.to_string() } y={ y.to_string() } width={ width.to_string() } height={ height.to_string() }
                        fill="none" stroke={ props.border_color.to_string() } stroke-width={ props.border_width.to_string() } />
                    { item }
                </ContextProvider<Frame>>
            }
        })
    }
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or(1)]
    pub rows: usize,
    #[prop_or(1)]
    pub cols: usize,
    #[prop_or(0)]
    pub border_width: i32,
    #[prop_or(Color::Black)]
    pub border_color: Color,
    #[prop_or_default]
    pub spacing: i32,
    #[prop_or_default]
    pub padding: i32,
}
