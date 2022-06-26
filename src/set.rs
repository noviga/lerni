use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Set)]
pub fn set(props: &Props) -> Html {
    html! {
        for props.children.iter()
    }
}
