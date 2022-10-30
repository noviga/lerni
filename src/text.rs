use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub text: String,
}

#[function_component(Text)]
pub fn text(props: &Props) -> Html {
    html! {
        <div class="has-ratio columns is-mobile is-vcentered has-background-danger">
            <div class="column has-text-centered">
                <p class="has-text-white is-size-1">{ props.text.clone() }</p>
            </div>
        </div>
    }
}
