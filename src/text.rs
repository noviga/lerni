use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
    #[prop_or_else(|| "#FFFFFF".to_string())]
    pub background: String,
}

#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    html! {
    <svg viewBox="0 0 640 360" class="has-ratio">
        <rect width="100%" height="100%" rx="10" fill={ props.background.clone() } />
        <text x="50%" y="50%" font-size="48" text-anchor="middle" dominant-baseline="middle">{ props.text.clone() }</text>
    </svg>
    }
}
