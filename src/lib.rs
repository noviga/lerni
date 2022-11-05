pub mod components;
pub mod layout;

use yew::Component;

pub fn start<T: Component>()
where
    <T as yew::Component>::Properties: Default,
{
    yew::start_app::<T>();
}
