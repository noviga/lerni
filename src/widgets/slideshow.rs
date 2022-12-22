use gloo_events::EventListener;
use std::collections::BTreeSet;
use wasm_bindgen::JsCast;
use yew::{html::Scope, prelude::*};

use crate::widgets::Metadata;

const KEY_ARROW_LEFT: u32 = 37;
const KEY_ARROW_RIGHT: u32 = 39;
const KEY_DIGIT_1: u32 = 49;
const KEY_DIGIT_9: u32 = KEY_DIGIT_1 + 8;
const BUTTON_COUNT: usize = 6;

/// Set of slides that are to be displayed sequentially.
#[derive(Clone, Default)]
pub struct SlideShow {
    current: usize,
    count: usize,
}

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub current: usize,
    #[prop_or_default]
    pub teacher_mode: bool,
    #[prop_or_default]
    pub pointer: bool,
}

pub enum Msg {
    Prev,
    Next,
    SetCurrent(usize),
}

impl Component for SlideShow {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let p = ctx.props();
        let doc = web_sys::window()
            .and_then(|win| win.document())
            .expect("Unable to get document");

        let link = ctx.link().clone();
        let event = EventListener::new(&doc, "keydown", move |e| {
            if let Some(e) = e.dyn_ref::<KeyboardEvent>() {
                match e.key_code() {
                    KEY_ARROW_LEFT => link.send_message(Msg::Prev),
                    KEY_ARROW_RIGHT => link.send_message(Msg::Next),
                    k @ (KEY_DIGIT_1..=KEY_DIGIT_9) => {
                        link.send_message(Msg::SetCurrent((k - KEY_DIGIT_1) as _))
                    }
                    _ => (),
                }
            }
        });
        event.forget();

        let count = p.children.len();
        let current = p.current.min(count - 1);
        Self { current, count }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
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
        let p = ctx.props();
        let link = ctx.link();

        html! {
            <>
                <div class="container pl-4 mt-4 pr-4">
                    { self.pagination(link) }
                </div>
                {
                    for p.children.iter().enumerate().map(|(i, item)| {
                        let metadata = Metadata {
                            visible: i == self.current,
                            teacher_mode: p.teacher_mode,
                            pointer: p.pointer,
                        };
                        html_nested! {
                            <g hidden={ i != self.current }>
                                <ContextProvider<Metadata> context={ metadata }>
                                    { item }
                                </ContextProvider<Metadata>>
                            </g>
                        }
                    })
                }
            </>
        }
    }
}

impl SlideShow {
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
    use super::{SlideShow, BUTTON_COUNT};

    #[test]
    fn page_list() {
        let c = BUTTON_COUNT;
        for i in 0..c {
            assert_eq!(SlideShow::page_list(i, c), (0..c).collect::<Vec<_>>());
        }

        let c = 2 * BUTTON_COUNT;
        assert_eq!(SlideShow::page_list(0, c), vec![0, 1, 2, c - 1]);
        assert_eq!(SlideShow::page_list(1, c), vec![0, 1, 2, c - 1]);
        assert_eq!(SlideShow::page_list(2, c), vec![0, 1, 2, 3, c - 1]);
        assert_eq!(SlideShow::page_list(3, c), vec![0, 2, 3, 4, c - 1]);
        assert_eq!(
            SlideShow::page_list(c - 4, c),
            vec![0, c - 5, c - 4, c - 3, c - 1]
        );
        assert_eq!(
            SlideShow::page_list(c - 3, c),
            vec![0, c - 4, c - 3, c - 2, c - 1]
        );
        assert_eq!(SlideShow::page_list(c - 2, c), vec![0, c - 3, c - 2, c - 1]);
        assert_eq!(SlideShow::page_list(c - 1, c), vec![0, c - 3, c - 2, c - 1]);
    }
}
