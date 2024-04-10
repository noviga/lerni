use leptos::{
    ev::{keydown, resize},
    *,
};
use leptos_use::*;
use std::collections::BTreeSet;
use web_sys::MouseEvent;

use crate::{PointerSignal, RefreshSignal};

const BUTTON_COUNT: usize = 6;
const PAGINATION_WIDTH: i32 = 40;
const CONTROL_PANEL_HEIGHT: i32 = 64;

#[component]
pub fn SlideShow(#[prop(optional)] current: usize, children: Children) -> impl IntoView {
    let refresh = create_rw_signal(());
    provide_context(RefreshSignal::new(refresh.read_only()));
    let pointer = create_rw_signal(true);
    provide_context(PointerSignal::new(pointer.read_only()));

    let page = create_rw_signal(current);
    let children = children().nodes;
    let count = children.len();

    _ = use_event_listener(document(), keydown, move |e| {
        if e.key() == "ArrowLeft" || e.key() == "ArrowUp" {
            if page.get() > 0 {
                page.set(page.get() - 1);
            }
        } else if (e.key() == "ArrowRight" || e.key() == "ArrowDown") && page.get() < count - 1 {
            page.set(page.get() + 1);
        }
    });

    let (width, set_width) =
        create_signal(crate::calc_width(PAGINATION_WIDTH, CONTROL_PANEL_HEIGHT));

    _ = use_event_listener(window(), resize, move |_| {
        set_width.set(crate::calc_width(PAGINATION_WIDTH, CONTROL_PANEL_HEIGHT));
    });

    let mut panel_items = Vec::with_capacity(count);
    let mut panel_item_refs = Vec::with_capacity(count);
    let children = children
        .into_iter()
        .enumerate()
        .map(|(i, child)| {
            let panel_item_ref = create_node_ref();
            panel_item_refs.push(panel_item_ref);
            panel_items
                .push(view! { <div node_ref=panel_item_ref hidden=move || i != page.get()></div> });
            view! {
                <div number=i hidden=move || i != page.get()>
                    {child}
                </div>
            }
        })
        .collect_view();

    provide_context(panel_item_refs);

    view! {
        <div
            class="container"
            style:max-width=move || {
                let width = width.get();
                if width > 0 { format!("{}px", width) } else { "100%".to_string() }
            }
        >

            <div class="columns is-gapless is-mobile">
                <div class="column is-rest">
                    {children} <ControlPanel refresh=refresh pointer=pointer>
                        {panel_items}
                    </ControlPanel>
                </div>
                <div class="pl-0 mt-4 pr-0" style:width="40px">
                    <Pagination page=page count=count/>
                </div>
            </div>
        </div>
    }
}

#[component]
fn ControlPanel(
    refresh: RwSignal<()>,
    pointer: RwSignal<bool>,
    children: Children,
) -> impl IntoView {
    view! {
        <div class="container pl-4 mt-4 pr-4" style="max-width: 100%;">
            <div class="field is-grouped">
                <p class="control">
                    <button class="button is-rounded is-link" on:click=history_back>
                        <span class="icon">
                            <i class="fas fa-lg fa-arrow-left"></i>
                        </span>
                    </button>
                </p>
                <p class="control">
                    <button
                        class="button is-rounded is-danger"
                        on:mousedown=move |e| e.prevent_default()
                        on:click=move |_| refresh.set(())
                    >
                        <span class="icon">
                            <i class="fas fa-lg fa-refresh"></i>
                        </span>
                    </button>
                </p>
                <p class="control">
                    <button
                        class="button is-rounded is-warning"
                        class:is-inverted=move || pointer.get()
                        on:mousedown=move |e| e.prevent_default()
                        on:click=move |_| pointer.update(|pointer| *pointer = !*pointer)
                    >
                        <span class="icon">
                            <i class="far fa-lg fa-dot-circle"></i>
                        </span>
                    </button>
                </p>
                {children()}
            </div>
        </div>
    }
}

#[component]
fn Pagination(page: RwSignal<usize>, count: usize) -> impl IntoView {
    let mut prev = None;
    let pages = (0..count)
        .map(|i| {
            let view = view! { <PageButton index=i count=count current=page/> };
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
        <div class="container">
            <nav
                class="pagination is-rounded is-flex is-flex-direction-column"
                role="navigation"
                aria-label="pagination"
            >
                <ul class="pagination-list mb-2 is-flex is-flex-direction-column is-align-items-center">
                    {pages}
                </ul>
                <a
                    class="pagination-previous button is-info mb-2"
                    style="order: 2;"
                    on:click=on_prev
                >
                    <span class="icon">
                        <i class="fas fa-lg fa-arrow-up"></i>
                    </span>
                </a>
                <a class="pagination-next button is-info" style="order: 3;" on:click=on_next>
                    <span class="icon">
                        <i class="fas fa-lg fa-arrow-down"></i>
                    </span>
                </a>
            </nav>
        </div>
    }
}

#[component]
fn PageButton(index: usize, count: usize, current: RwSignal<usize>) -> impl IntoView {
    view! {
        <>
            <li hidden=move || {
                index == 0 || is_page_number_visible(index - 1, current.get(), count)
                    || !is_page_number_visible(index, current.get(), count)
            }>
                <span class="icon">
                    <i class="fas fa-lg fa-ellipsis-v"></i>
                </span>
            </li>
            <li hidden=move || !is_page_number_visible(index, current.get(), count)>
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

fn is_page_number_visible(index: usize, current: usize, count: usize) -> bool {
    if count <= BUTTON_COUNT {
        true
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
        pages.contains(&index)
    }
}

fn history_back(_event: MouseEvent) {
    if let Some(window) = web_sys::window() {
        if let Ok(history) = window.history() {
            history.back().unwrap_or_default();
        }
    }
}
