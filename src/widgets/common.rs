use std::any::Any;
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

pub trait Widget2: Any + WidgetClone {
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32);
    fn render(&self) -> Html;
}

pub type WidgetObject = Box<dyn Widget2>;

pub trait WidgetClone {
    fn clone_box(&self) -> WidgetObject;
}

impl<T> WidgetClone for T
where
    T: 'static + Widget2 + Clone,
{
    fn clone_box(&self) -> WidgetObject {
        Box::new(self.clone())
    }
}

impl Clone for WidgetObject {
    fn clone(&self) -> Self {
        self.clone_box()
    }
}

impl PartialEq for WidgetObject {
    fn eq(&self, other: &Self) -> bool {
        self.type_id() == other.type_id()
    }
}

impl From<WidgetObject> for Html {
    fn from(widget: WidgetObject) -> Self {
        widget.render()
    }
}
