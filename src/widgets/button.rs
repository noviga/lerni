use yew::{html::Scope, prelude::*};

use crate::{
    properties::Color,
    widgets::{Label, Widget},
};

const WIDTH: usize = 400;
const HEIGHT: usize = 150;

/// Button widget.
#[derive(Default)]
pub struct Button {
    props: Props,
    mouse_down: bool,
}

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
    #[prop_or(Color::AliceBlue)]
    color: Color,
    #[prop_or(12)]
    border_width: usize,
    #[prop_or(Color::RoyalBlue)]
    border_color: Color,
    #[prop_or_default]
    pub onclick: Callback<(Props, Scope<Button>)>,
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
    OnMouseDown,
    OnMouseUp,
    /// Changes the button's text.
    SetText(String),
    /// Changes the button's color.
    SetColor(Color),
    /// Changes the button's border color.
    SetBorderColor(Color),
}

impl Component for Button {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::OnMouseDown => {
                self.mouse_down = true;
                true
            }
            Msg::OnMouseUp => {
                self.mouse_down = false;
                ctx.props()
                    .onclick
                    .emit((ctx.props().clone(), ctx.link().clone()));
                true
            }
            Msg::SetText(text) => {
                self.props.text = text;
                true
            }
            Msg::SetColor(color) => {
                self.props.color = color;
                true
            }
            Msg::SetBorderColor(color) => {
                self.props.border_color = color;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = &self.props;
        let x = (p.x + p.border_width / 2).to_string();
        let y = (p.y + p.border_width / 2).to_string();
        let width = (p.width - p.border_width).to_string();
        let height = (p.height - p.border_width).to_string();
        let lx = p.x + p.width / 2;
        let ly = p.y + p.height / 2;

        let border_width = if self.mouse_down {
            p.border_width + 6
        } else {
            p.border_width
        };

        html! {
            <a onmousedown={ ctx.link().callback(|_| Msg::OnMouseDown) }
                onmouseup={ ctx.link().callback(|_| Msg::OnMouseUp) }>
                <rect { x } { y } { width } { height }
                    rx={ p.radius.to_string() } ry={ p.radius.to_string() }
                    fill={ p.color.to_string() } stroke={ p.border_color.to_string() }
                    stroke-width={ border_width.to_string() } />
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

    /// Adds a callback for setting the button's color.
    ///
    /// The callback's parameter is the property color (see [`Props`]).
    pub fn set_color(mut self, f: impl Fn(&Color) -> Color + 'static) -> Self {
        self.fns
            .push(Box::new(move |p: &Props| Msg::SetColor(f(&p.color))));
        self
    }

    /// Adds a callback for setting the button's border color.
    ///
    /// The callback's parameter is the property border color (see [`Props`]).
    pub fn set_border_color(mut self, f: impl Fn(&Color) -> Color + 'static) -> Self {
        self.fns
            .push(Box::new(move |p: &Props| Msg::SetBorderColor(f(&p.color))));
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
        ..Default::default()
    })
}
