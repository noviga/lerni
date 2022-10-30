use gloo_events::EventListener;
use wasm_bindgen::JsCast;
use yew::prelude::*;

const ARROW_LEFT_KEY: u32 = 37;
const ARROW_RIGHT_KEY: u32 = 39;

pub enum Msg {
    Prev,
    Next,
    SetCurrent(usize),
}

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Children,
    #[prop_or_default]
    pub current: usize,
}

pub struct Cards {
    current: usize,
}

impl Component for Cards {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let doc = web_sys::window()
            .and_then(|win| win.document())
            .expect("Unable to get document");

        let link = ctx.link().clone();
        let event = EventListener::new(&doc, "keydown", move |e| {
            if let Some(e) = e.dyn_ref::<KeyboardEvent>() {
                match e.key_code() {
                    ARROW_LEFT_KEY => link.send_message(Msg::Prev),
                    ARROW_RIGHT_KEY => link.send_message(Msg::Next),
                    _ => (),
                }
            }
        });
        event.forget();

        Cards {
            current: ctx.props().current,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let max = ctx.props().children.len() - 1;
        match msg {
            Msg::Prev if self.current > 0 => self.current -= 1,
            Msg::Next if self.current < max => self.current += 1,
            Msg::SetCurrent(c) => self.current = c,
            _ => return false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let props = ctx.props();
        let link = ctx.link();

        html! {
            <div class="container pl-4 mt-4" >
                <nav class="pagination is-rounded" role="navigation" aria-label="pagination">
                    <ul class="pagination-list">
                        {
                            for props.children.iter().enumerate().map(|(i, _)| {
                                let class = if i == self.current {
                                    "pagination-link button is-warning"
                                } else {
                                    "pagination-link"
                                };
                                html! {
                                    <li><a class={ class } onclick={ link.callback(move |_| Msg::SetCurrent(i)) }>
                                        { i + 1 }
                                    </a></li>
                                }

                            })
                        }
                    </ul>
                    <a class="pagination-previous button is-info" onclick={ link.callback(|_| Msg::Prev) }>
                        <span class="icon"><i class="fas fa-lg fa-arrow-left"></i></span>
                    </a>
                    <a class="pagination-next button is-info mr-4" onclick={ link.callback(|_| Msg::Next) }>
                        <span class="icon"><i class="fas fa-lg fa-arrow-right"></i></span>
                    </a>
                </nav>
                <div class="ml-4">
                    <figure class="image is-16by9">
                        { props.children.iter().nth(self.current).unwrap() }
                    </figure>
                </div>
            </div>
        }
    }
}
