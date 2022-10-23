#![allow(clippy::derive_partial_eq_without_eq)]

use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub current: usize,
}

#[function_component(Cards)]
pub fn cards(props: &Props) -> Html {
    html! {
        <div class="container mt-4">
            <nav class="pagination is-rounded" role="navigation">
                <a class="pagination-previous button is-info">
                    <span class="icon"><i class="fas fa-lg fa-solid fa-arrow-left"></i></span>
                </a>
                <a class="pagination-next button is-info">
                    <span class="icon"><i class="fas fa-lg fa-solid fa-arrow-right"></i></span>
                </a>
                <ul class="pagination-list">
                    { 
                        for props.children.iter().enumerate().map(|(i, _)| {
                            let qwe = if i == props.current {
                                "pagination-link button is-warning"
                            } else {
                                "pagination-link"
                            };
                            html! {
                                <li><a class={ qwe }>{ i + 1 }</a></li>
                            }
                            
                        })
                    }
                </ul>
            </nav>
            <div class="box pr-0 pb-0">  
                <figure class="image is-16by9 pr-0">
                    { props.children.iter().nth(props.current.min(props.children.len() - 1)).unwrap() }
                </figure>
            </div>
        </div>
    }
}
