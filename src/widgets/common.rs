use std::any::Any;
use yew::Html;

/// Trait that all widget are to implement.
pub trait Widget: Any + WidgetClone {
    /// Set frame within which the widget should be rendered.
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32);
    /// Render the widget.
    fn render(&self) -> Html;
}

/// Type alias to Widget trait object.
pub type WidgetObject = Box<dyn Widget>;

pub trait WidgetClone {
    fn clone_box(&self) -> WidgetObject;
}

impl<T> WidgetClone for T
where
    T: 'static + Widget + Clone,
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
