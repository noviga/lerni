use yew::{html::Scope, prelude::*};

use super::{common::Widget, Label};

const WIDTH: usize = 400;
const HEIGHT: usize = 150;

#[derive(Default, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Button".to_string())]
    pub text: String,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    #[prop_or(24)]
    radius: usize,
    #[prop_or_else(|| "orange".to_string())]
    color: String,
    #[prop_or(12)]
    border_width: usize,
    #[prop_or_else(|| "blue".to_string())]
    border_color: String,
    #[prop_or_default]
    pub onclick: Callback<(Props, Scope<Button>)>,
}

/// Button widget.
#[derive(Default)]
pub struct Button {
    props: Props,
}

impl Widget for Button {
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html {
        let x = x + width / 2 - WIDTH / 2;
        let y = y + height / 2 - HEIGHT / 2;
        html! {
            <Button text={ self.props.text.clone() } { x } { y } width={ WIDTH } height={ HEIGHT }
                onclick={ &self.props.onclick } />
        }
    }
}

/// Button messages.
pub enum Msg {
    /// Is sent to the button when clicked.
    OnClick,
    /// Changes the button's text.
    SetText(String),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
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
                self.props.text = text;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let style = format!(
            "fill:{};stroke:{};stroke-width:{}",
            p.color, p.border_color, p.border_width
        );
        let x = (p.x + p.border_width / 2).to_string();
        let y = (p.y + p.border_width / 2).to_string();
        let width = (p.width - p.border_width).to_string();
        let height = (p.height - p.border_width).to_string();
        let lx = p.x + p.width / 2;
        let ly = p.y + p.height / 2;
        html! {
            <a onclick={ ctx.link().callback(|_| Msg::OnClick) }>
                <rect { x } { y } { width } { height }
                    rx={ p.radius.to_string() } ry={ p.radius.to_string() } { style } />
                <Label x={ lx } y={ ly } text={ self.props.text.clone() } />
            </a>
        }
    }
}

type CallbackItem = Box<dyn Fn(&Props) -> Msg>;

#[derive(Default)]
pub struct ButtonCallback {
    fns: Vec<CallbackItem>,
}

impl ButtonCallback {
    /// Adds a callback for setting the button's text.
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

impl Button {
    /// Returns callback builder struct.
    pub fn callback() -> ButtonCallback {
        Default::default()
    }

    /// Sets `onclick` event handler.
    pub fn onclick(mut self: Box<Self>, callback: Callback<(Props, Scope<Button>)>) -> Box<Self> {
        self.props.onclick = callback;
        self
    }
}

/// Creates a new `Button` widget.
pub fn button(text: &str) -> Box<Button> {
    Box::new(Button {
        props: Props {
            text: text.to_string(),
            ..Default::default()
        },
    })
}
