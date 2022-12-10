use yew::prelude::*;

use crate::widgets::Frame;

/// Row of widgets.
#[function_component]
pub fn Row(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let rows = props.children.len() as i32;
    let width = f.width / rows;

    html! {
        for props.children.iter().enumerate().map(|(i, item)| {
            let x = f.x + width * i as i32;
            let frame = Frame {
                x,
                y: f.y,
                width,
                height: f.height,
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
