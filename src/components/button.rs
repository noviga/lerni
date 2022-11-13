use yew::html::Scope;
use yew::prelude::*;

use crate::components::Text;

/// Button messages.
#[derive(Clone)]
pub enum Msg {
    /// Is sent to the button when clicked.
    OnClick,
    /// Changes the button's text.
    SetText(String),
}

#[derive(Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Button".to_string())]
    pub text: String,
    #[prop_or_default]
    pub onclick: Callback<(Props, Scope<Button>)>,
}

/// Button component.
pub struct Button {
    text: String,
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;
    fn create(ctx: &Context<Self>) -> Self {
        Self {
            text: ctx.props().text.clone(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnClick => {
                ctx.props()
                    .onclick
                    .emit((ctx.props().clone(), ctx.link().clone()));
                false
            }
            Msg::SetText(text) => {
                self.text = text;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <a onclick={ ctx.link().callback(|_| Msg::OnClick) }>
                <rect x="20%" y="30%" rx="20" ry="20" width="60%" height="40%"
                    style="fill:red;stroke:black;stroke-width:10;opacity:0.5" />
                <Text>{ &self.text }</Text>
            </a>
        }
    }
}

impl Button {
    /// Return callback builder struct.
    pub fn callback() -> ButtonCallback {
        Default::default()
    }
}

type CallbackItem = Box<dyn Fn(&Props) -> Msg>;

#[derive(Default)]
pub struct ButtonCallback {
    fns: Vec<CallbackItem>,
}

impl ButtonCallback {
    /// Add a callback for setting the button's text.
    ///
    /// The callback's parameter is the property text (see [`Props`]).
    pub fn set_text(mut self, f: impl Fn(&str) -> String + 'static) -> Self {
        self.fns
            .push(Box::new(move |p: &Props| Msg::SetText(f(&p.text))));
        self
    }

    /// Builds a buttons callback with a message batch collected earlier.
    pub fn build(self) -> Callback<(Props, Scope<Button>)> {
        Callback::from(move |(props, link): (Props, Scope<Button>)| {
            link.send_message_batch(self.fns.iter().map(|f| f(&props)).collect());
        })
    }
}
