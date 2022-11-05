use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_else(|| "#FFFFFF".to_string())]
    pub background: String,
}

#[function_component(Frame)]
pub fn frame(props: &Props) -> Html {
    html! {
        <svg viewBox="0 0 640 360" class="has-ratio">
            <rect width="100%" height="100%" rx="10" fill={ props.background.clone() } />
            { for props.children.iter() }
        </svg>
    }
}
