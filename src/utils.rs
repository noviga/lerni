use leptos::*;

/// Start function.
///
/// # Example
///
/// ```no_run
/// use leptos::*;
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
    leptos::mount_to_body(f);
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
