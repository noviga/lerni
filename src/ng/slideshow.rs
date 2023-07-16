use leptos::*;

use crate::ng::Metadata;

#[component]
pub fn SlideShow(
    cx: Scope,
    #[prop(optional)] current: usize,
    #[prop(optional)] teacher_mode: bool,
    #[prop(optional)] pointer: bool,
    children: Children,
) -> impl IntoView {
    let mut metadata = Metadata {
        teacher_mode,
        pointer,
        ..Default::default()
    };

    let children = children(cx)
        .nodes
        .into_iter()
        .enumerate()
        .map(|(i, child)| {
            metadata.visible = i == current;
            provide_context(cx, metadata);
            view! { cx, <div hidden={i != current}>{child}</div> }
        })
        .collect_view(cx);

    view! { cx,
        <>
            // TODO: add pagination
            {children}
        </>
    }
}
