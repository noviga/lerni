use std::{any::Any, rc::Rc};
use yew::{virtual_dom::VChild, Component, Html};

/// Trait that all widget are to implement.
pub trait Widget: Any + WidgetClone {
    /// Set frame within which the widget should be rendered.
    #[allow(unused_variables)]
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32) {}
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

/// Trait for creating widget from properties.
pub trait FromProperties: Component {
    /// Creates `Component` from properties.
    fn from_properties(props: Rc<Self::Properties>) -> Self;
}

impl<T> From<VChild<T>> for WidgetObject
where
    T: Component + FromProperties + Widget,
{
    fn from(child: VChild<T>) -> Self {
        Box::new(T::from_properties(child.props))
    }
}
