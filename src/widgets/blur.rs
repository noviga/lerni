use yew::prelude::*;

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or(15)]
    pub radius: i32,
    #[prop_or(0.3)]
    pub delay: f32,
}

/// Blur widget.
#[function_component]
pub fn Blur(props: &Props) -> Html {
    let radius = if props.disabled { 0 } else { props.radius };
    let style = format!(
        "filter: blur({radius}px); transition: all {}s;",
        props.delay
    );
    html! {
        <g { style }>
            { for props.children.iter() }
        </g>
    }
}
