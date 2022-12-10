use yew::prelude::*;

use crate::widgets::Frame;

/// Column of widgets.
#[function_component]
pub fn Column(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();
    let rows = props.children.len() as i32;
    let height = f.height / rows;

    html! {
        for props.children.iter().enumerate().map(|(i, item)| {
            let y = f.y + height * i as i32;
            let frame = Frame {
                x: f.x,
                y,
                width: f.width,
                height,
            };
            html_nested! {
                <ContextProvider<Frame> context={ frame }>
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
}
