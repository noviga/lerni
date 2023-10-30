use leptos::{
    ev::{keydown, resize},
    *,
};
use leptos_use::*;
use std::collections::BTreeSet;

use crate::Metadata;

const BUTTON_COUNT: usize = 6;
const PAGINATION_HEIGHT: i32 = 88;

#[component]
pub fn SlideShow(
    #[prop(optional)] current: usize,
    #[prop(optional)] teacher_mode: bool,
    #[prop(optional)] pointer: bool,
    children: Children,
) -> impl IntoView {
    let page = create_rw_signal(current);
    let children = children().nodes;
    let count = children.len();

    let _ = use_event_listener(document(), keydown, move |e| {
        if e.key() == "ArrowLeft" {
            if page.get() > 0 {
                page.set(page.get() - 1);
            }
        } else if e.key() == "ArrowRight" && page.get() < count - 1 {
            page.set(page.get() + 1);
        }
    });

    let (width, set_width) = create_signal(crate::calc_width(PAGINATION_HEIGHT));

    let _ = use_event_listener(window(), resize, move |_| {
        set_width.set(crate::calc_width(PAGINATION_HEIGHT));
    });

    let mut metadata = Metadata {
        teacher_mode,
        pointer,
        ..Default::default()
    };

    let children = children
        .into_iter()
        .enumerate()
        .map(|(i, child)| {
            metadata.visible = i == page.get();
            provide_context(metadata);
            view! { <div hidden=move || i != page.get()>{child}</div> }
        })
        .collect_view();

    view! {
        <div
            class="container"
            style:max-width=move || {
                let width = width.get();
                if width > 0 { format!("{}px", width) } else { "100%".to_string() }
            }
        >

            <Pagination page=page count=count/>
            {children}
        </div>
    }
}

#[component]
fn Pagination(page: RwSignal<usize>, count: usize) -> impl IntoView {
    let mut prev = None;
    let pages = page_list(page.get(), count)
        .into_iter()
        .map(|i| {
            let view = view! { <PageButton index=i prev=prev current=page/> };
            prev = Some(i);
            view
        })
        .collect_view();

    let on_prev = move |_| {
        if page.get() > 0 {
            page.set(page.get() - 1);
        }
    };
    let on_next = move |_| {
        if page.get() < count - 1 {
            page.set(page.get() + 1);
        }
    };

    view! {
        <div class="container pl-4 mt-4 pr-4" style="max-width: 100%;">
            <nav class="pagination is-rounded" role="navigation" aria-label="pagination">
                <ul class="pagination-list">{pages}</ul>
                <a class="pagination-link button is-info" style="order: 2;" on:click=on_prev>
                    <span class="icon">
                        <i class="fas fa-lg fa-arrow-left"></i>
                    </span>
                </a>
                <a class="pagination-link button is-info" style="order: 3;" on:click=on_next>
                    <span class="icon">
                        <i class="fas fa-lg fa-arrow-right"></i>
                    </span>
                </a>
            </nav>
        </div>
    }
}

#[component]
fn PageButton(index: usize, prev: Option<usize>, current: RwSignal<usize>) -> impl IntoView {
    view! {
        <>
            <li hidden=move || !matches!(prev, Some(p) if index != (p + 1))>
                <span class="pagination-ellipsis">"•"</span>
            </li>
            <li>
                <a
                    class="pagination-link"
                    class:button=move || index == current.get()
                    class:is-warning=move || index == current.get()
                    on:click=move |_| current.set(index)
                >
                    {index + 1}
                </a>
            </li>
        </>
    }
}

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_list_calc() {
        let c = BUTTON_COUNT;
        for i in 0..c {
            assert_eq!(page_list(i, c), (0..c).collect::<Vec<_>>());
        }

        let c = 2 * BUTTON_COUNT;
        assert_eq!(page_list(0, c), vec![0, 1, 2, c - 1]);
        assert_eq!(page_list(1, c), vec![0, 1, 2, c - 1]);
        assert_eq!(page_list(2, c), vec![0, 1, 2, 3, c - 1]);
        assert_eq!(page_list(3, c), vec![0, 2, 3, 4, c - 1]);
        assert_eq!(page_list(c - 4, c), vec![0, c - 5, c - 4, c - 3, c - 1]);
        assert_eq!(page_list(c - 3, c), vec![0, c - 4, c - 3, c - 2, c - 1]);
        assert_eq!(page_list(c - 2, c), vec![0, c - 3, c - 2, c - 1]);
        assert_eq!(page_list(c - 1, c), vec![0, c - 3, c - 2, c - 1]);
    }
}
