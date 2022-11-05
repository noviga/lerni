use gloo_events::EventListener;
use std::collections::BTreeSet;
use wasm_bindgen::JsCast;
use yew::{html::Scope, prelude::*};

const KEY_ARROW_LEFT: u32 = 37;
const KEY_ARROW_RIGHT: u32 = 39;
const KEY_DIGIT_1: u32 = 49;
const KEY_DIGIT_9: u32 = KEY_DIGIT_1 + 8;
const BUTTON_COUNT: usize = 6;

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
    count: usize,
}

impl Component for Cards {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let doc = web_sys::window()
            .and_then(|win| win.document())
            .expect("Unable to get document");

        let scope = ctx.link().clone();
        let event = EventListener::new(&doc, "keydown", move |e| {
            if let Some(e) = e.dyn_ref::<KeyboardEvent>() {
                match e.key_code() {
                    KEY_ARROW_LEFT => scope.send_message(Msg::Prev),
                    KEY_ARROW_RIGHT => scope.send_message(Msg::Next),
                    k @ (KEY_DIGIT_1..=KEY_DIGIT_9) => {
                        scope.send_message(Msg::SetCurrent((k - KEY_DIGIT_1) as _))
                    }
                    _ => (),
                }
            }
        });
        event.forget();

        Cards {
            current: ctx.props().current,
            count: ctx.props().children.len(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        if self.count == 0 {
            return false;
        }
        let max = self.count - 1;
        match msg {
            Msg::Prev if self.current > 0 => self.current -= 1,
            Msg::Next if self.current < max => self.current += 1,
            Msg::SetCurrent(c) if c <= max => self.current = c,
            _ => return false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        if self.count == 0 {
            return html!();
        }
        let props = ctx.props();
        let scope = ctx.link();

        html! {
            <div class="container pl-4 mt-4 pr-4">
                { self.pagination(scope) }
                <div class="box">
                    <figure class="image is-16by9">
                        { props.children.iter().nth(self.current).unwrap() }
                    </figure>
                </div>
            </div>
        }
    }
}

impl Cards {
    fn page_list(current: usize, count: usize) -> Vec<usize> {
        if count <= BUTTON_COUNT {
            (0..count).collect()
        } else {
            let mut pages: BTreeSet<usize> = [0].into();
            let mut add_page = |page| {
                if page < count {
                    pages.insert(page);
                }
            };
            add_page(count - 1);
            let center = if current == 0 {
                1
            } else if current == count - 1 {
                count - 2
            } else {
                current
            };
            add_page(center - 1);
            add_page(center);
            add_page(center + 1);
            pages.into_iter().collect()
        }
    }

    fn page_button(&self, prev: Option<usize>, index: usize, scope: &Scope<Self>) -> Html {
        let class = if index == self.current {
            "pagination-link button is-warning"
        } else {
            "pagination-link"
        };

        let button = html! {
            <li><a class={ class } onclick={ scope.callback(move |_| Msg::SetCurrent(index)) }>
                { index + 1 }
            </a></li>
        };

        if matches!(prev, Some(p) if index != (p + 1)) {
            html!(<><li><span class="pagination-ellipsis">{ '…' }</span></li>{ button }</>)
        } else {
            button
        }
    }

    fn pagination(&self, scope: &Scope<Self>) -> Html {
        let pages = Self::page_list(self.current, self.count);
        let mut prev = None;

        html! {
            <nav class="pagination is-rounded" role="navigation" aria-label="pagination">
                <ul class="pagination-list">
                {
                    for pages.into_iter().map(|i| {
                        let html = self.page_button(prev, i, scope);
                        prev = Some(i);
                        html
                    })
                }
                </ul>
                <a class="pagination-previous button is-info" onclick={ scope.callback(|_| Msg::Prev) }>
                    <span class="icon"><i class="fas fa-lg fa-arrow-left"></i></span>
                </a>
                <a class="pagination-next button is-info" onclick={ scope.callback(|_| Msg::Next) }>
                    <span class="icon"><i class="fas fa-lg fa-arrow-right"></i></span>
                </a>
            </nav>
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{Cards, BUTTON_COUNT};

    #[test]
    fn page_list() {
        let c = BUTTON_COUNT;
        for i in 0..c {
            assert_eq!(Cards::page_list(i, c), (0..c).collect::<Vec<_>>());
        }

        let c = 2 * BUTTON_COUNT;
        assert_eq!(Cards::page_list(0, c), vec![0, 1, 2, c - 1]);
        assert_eq!(Cards::page_list(1, c), vec![0, 1, 2, c - 1]);
        assert_eq!(Cards::page_list(2, c), vec![0, 1, 2, 3, c - 1]);
        assert_eq!(Cards::page_list(3, c), vec![0, 2, 3, 4, c - 1]);
        assert_eq!(
            Cards::page_list(c - 4, c),
            vec![0, c - 5, c - 4, c - 3, c - 1]
        );
        assert_eq!(
            Cards::page_list(c - 3, c),
            vec![0, c - 4, c - 3, c - 2, c - 1]
        );
        assert_eq!(Cards::page_list(c - 2, c), vec![0, c - 3, c - 2, c - 1]);
        assert_eq!(Cards::page_list(c - 1, c), vec![0, c - 3, c - 2, c - 1]);
    }
}
