use yew::Html;

/// Trait that all widget are to implement.
pub trait Widget {
    /// Renders a widget within a rectangle.
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html;
}

/// Trait for multi-widget.
///
/// Multi-widget is a widget that contains child widgets.
pub trait MultiWidget: Widget {
    /// Returns list of child widgets.
    fn children(&self) -> &[Box<dyn Widget>];

    /// Returns mutable list of child widgets.
    fn children_mut(&mut self) -> &mut Vec<Box<dyn Widget>>;

    /// Adds a new widget to the list of child widgets and returns boxed `Self`.
    fn add(mut self: Box<Self>, item: Box<dyn Widget>) -> Box<Self> {
        self.children_mut().push(item);
        self
    }
}
