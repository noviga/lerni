use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    html! {
        props.text.clone()
    }
}
