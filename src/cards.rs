use yew::prelude::*;

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub current: usize,
}

#[function_component(Cards)]
pub fn cards(props: &Props) -> Html {
    let card = use_state(|| props.current);
    let max_card = props.children.len() - 1;

    let prev_card = {
        let card = card.clone();
        Callback::from(move |_| {
            if *card > 0 {
                card.set(*card - 1)
            }
        })
    };

    let next_card = {
        let card = card.clone();
        Callback::from(move |_| {
            if *card < max_card {
                card.set(*card + 1)
            }
        })
    };

    html! {
        <div class="container pl-4 mt-4">
            <nav class="pagination is-rounded" role="navigation" aria-label="pagination">
                <ul class="pagination-list">
                    {
                        for props.children.iter().enumerate().map(|(i, _)| {
                            let class = if i == *card {
                                "pagination-link button is-warning"
                            } else {
                                "pagination-link"
                            };
                            html! {
                                <li><a class={ class } onclick={ let card = card.clone(); Callback::from(move |_| card.set(i)) }>{ i + 1 }</a></li>
                            }

                        })
                    }
                </ul>
                <a class="pagination-previous button is-info" onclick={ prev_card }>
                    <span class="icon"><i class="fas fa-lg fa-solid fa-arrow-left"></i></span>
                </a>
                <a class="pagination-next button is-info mr-4" onclick={ next_card }>
                    <span class="icon"><i class="fas fa-lg fa-solid fa-arrow-right"></i></span>
                </a>
            </nav>
            <div class="ml-4">
                <figure class="image is-16by9">
                    { props.children.iter().nth(*card).unwrap() }
                </figure>
            </div>
        </div>
    }
}
