use leptos::{html::Div, mount, prelude::*};

/// Start function.
///
/// # Example
///
/// ```no_run
/// use leptos::prelude::*;
/// use lerni::*;
///
/// #[component]
/// pub fn HelloWorld() -> impl IntoView {
///     view! {
///         "Hello, world!"
///     }
/// }
///
/// #[wasm_bindgen(start)]
/// pub fn main() {
///     lerni::start(HelloWorld);
/// }
/// ```
pub fn start<F, N>(f: F)
where
    F: Fn() -> N + 'static,
    N: IntoView,
{
    mount::mount_to_body(f);
}

/// Keyboard key codes.
pub mod keys {
    /// Backspace key.
    pub const BACKSPACE: u32 = 8;
    /// Tab key.
    pub const TAB: u32 = 9;
    /// Enter key.
    pub const ENTER: u32 = 13;
    /// Escape key.
    pub const ESCAPE: u32 = 27;
    /// Space key.
    pub const SPACE: u32 = 32;
    /// Left arrow key.
    pub const ARROW_LEFT: u32 = 37;
    /// Up arrow key.
    pub const ARROW_UP: u32 = 38;
    /// Right arrow key.
    pub const ARROW_RIGHT: u32 = 39;
    /// Down arrow key.
    pub const ARROW_DOWN: u32 = 40;
    /// Digit 0 key.
    pub const DIGIT_0: u32 = 48;
    /// Digit 1 key.
    pub const DIGIT_1: u32 = 49;
    /// Digit 2 key.
    pub const DIGIT_2: u32 = 50;
    /// Digit 3 key.
    pub const DIGIT_3: u32 = 51;
    /// Digit 4 key.
    pub const DIGIT_4: u32 = 52;
    /// Digit 5 key.
    pub const DIGIT_5: u32 = 53;
    /// Digit 6 key.
    pub const DIGIT_6: u32 = 54;
    /// Digit 7 key.
    pub const DIGIT_7: u32 = 55;
    /// Digit 8 key.
    pub const DIGIT_8: u32 = 56;
    /// Digit 9 key.
    pub const DIGIT_9: u32 = 57;
}

/// Check whether a slide is active.
pub fn is_active_slide(node_ref: NodeRef<Div>) -> bool {
    if let Some(node) = node_ref.get() {
        if let Some(parent) = node.parent_element() {
            return parent.get_attribute("hidden").is_none();
        }
    }

    false
}

/// Check whether a slide is visible.
pub fn slide_number(node_ref: NodeRef<Div>) -> Option<usize> {
    if let Some(node) = node_ref.get_untracked() {
        if let Some(parent) = node.parent_element() {
            return parent.get_attribute("number").and_then(|n| n.parse().ok());
        }
    }

    None
}

/// Mount a child view on a panel.
pub fn mount_on_panel(node_ref: NodeRef<Div>, _item: AnyView) {
    let panel_item_refs: Option<Vec<NodeRef<Div>>> = use_context();
    if let Some(panel_item_refs) = panel_item_refs {
        if let Some(n) = slide_number(node_ref) {
            if let Some(panel_item_ref) = panel_item_refs.get(n) {
                if let Some(_el) = panel_item_ref.get_untracked() {
                    //el.append_child(item);
                }
            }
        }
    }
}
