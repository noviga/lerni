use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
}

#[function_component(Slide)]
pub fn slide(props: &Props) -> Html {
    html! {
        <div class="container pl-4 mt-4 pr-4">
            <div class="box">
                <figure class="image is-16by9">
                    { for props.children.iter() }
                </figure>
            </div>
        </div>
    }
}
