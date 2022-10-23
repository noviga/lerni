mod cards;
mod text;

use yew::Component;

pub use crate::{cards::Cards, text::Text};

pub fn start<T: Component>()
where
    <T as yew::Component>::Properties: Default,
{
    yew::start_app::<T>();
}
