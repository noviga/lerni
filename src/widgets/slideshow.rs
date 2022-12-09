use gloo_events::EventListener;
use std::{collections::BTreeSet, rc::Rc};
use wasm_bindgen::JsCast;
use yew::{
    html::{ChildrenRenderer, Scope},
    prelude::*,
    virtual_dom::VChild,
};

use crate::widgets::{Widget, WidgetObject};

const WIDTH: i32 = 1920;
const HEIGHT: i32 = 1080;

const KEY_ARROW_LEFT: u32 = 37;
const KEY_ARROW_RIGHT: u32 = 39;
const KEY_DIGIT_1: u32 = 49;
const KEY_DIGIT_9: u32 = KEY_DIGIT_1 + 8;
const BUTTON_COUNT: usize = 6;

#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: ChildrenRenderer<WidgetObject>,
    #[prop_or(WIDTH)]
    pub width: i32,
    #[prop_or(HEIGHT)]
    pub height: i32,
    #[prop_or_default]
    pub current: usize,
}

/// Set of slides that are to be displayed sequentially.
#[derive(Clone, Default)]
pub struct SlideShow {
    count: usize,
    props: Rc<Props>,
}

pub enum Msg {
    Prev,
    Next,
    SetCurrent(usize),
}

impl Widget for SlideShow {
    fn set_frame(&mut self, _x: i32, _y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.width = width;
        p.height = height;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        html! {
            <SlideShow width={ p.width } height={ p.height } current={ p.current }>
                { for p.children.iter() }
            </SlideShow>
        }
    }
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
        let mut props = p.clone();
        props.current = p.current.min(count - 1);
        Self {
            count,
            props: Rc::new(props),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        let p = Rc::make_mut(&mut self.props);
        let max = self.count - 1;
        match msg {
            Msg::Prev if p.current > 0 => p.current -= 1,
            Msg::Next if p.current < max => p.current += 1,
            Msg::SetCurrent(c) if c <= max => p.current = c,
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
                    p.children.iter().nth(self.props.current).map(|mut item|{
                        item.set_frame(0, 0, p.width, p.height);
                        item
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
        let class = if index == self.props.current {
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
        let pages = Self::page_list(self.props.current, self.count);
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

impl From<VChild<SlideShow>> for WidgetObject {
    fn from(child: VChild<SlideShow>) -> Self {
        Box::new(SlideShow {
            count: child.props.children.len(),
            props: child.props,
        })
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
