use std::rc::Rc;
use yew::{html::Scope, prelude::*, virtual_dom::VChild};

use crate::{
    properties::Color,
    widgets::{Label, Widget, WidgetObject},
};

const WIDTH: i32 = 400;
const HEIGHT: i32 = 150;

/// Button widget.
#[derive(Clone, Default)]
pub struct Button {
    props: Rc<Props>,
    mouse_down: bool,
}

#[derive(Default, Clone, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Button".to_string())]
    pub text: String,
    #[prop_or_default]
    pub x: i32,
    #[prop_or_default]
    pub y: i32,
    #[prop_or_default]
    pub width: i32,
    #[prop_or_default]
    pub height: i32,
    #[prop_or(24)]
    pub radius: i32,
    #[prop_or(Color::AliceBlue)]
    pub color: Color,
    #[prop_or(12)]
    pub border_width: i32,
    #[prop_or(Color::RoyalBlue)]
    pub border_color: Color,
    #[prop_or_default]
    pub onclick: Callback<(Props, Scope<Button>)>,
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
            props: Rc::new(ctx.props().clone()),
            ..Default::default()
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let p = Rc::make_mut(&mut self.props);
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
                p.text = text;
                true
            }
            Msg::SetColor(color) => {
                p.color = color;
                true
            }
            Msg::SetBorderColor(color) => {
                p.border_color = color;
                true
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
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
                    rx={ self.props.radius.to_string() } ry={ self.props.radius.to_string() }
                    fill={ self.props.color.to_string() } stroke={ self.props.border_color.to_string() }
                    stroke-width={ border_width.to_string() } />
                <Label x={ lx } y={ ly } text={ self.props.text.clone() } />
            </a>
        }
    }
}

impl Widget for Button {
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.width = WIDTH;
        p.height = HEIGHT;
        p.x = x + width / 2 - p.width / 2;
        p.y = y + height / 2 - p.height / 2;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        html! {
            <Button text={ p.text.clone() } x={ p.x } y={ p.y } width={ p.width } height={ p.height }
                onclick={ &self.props.onclick } />
        }
    }
}

impl From<VChild<Button>> for WidgetObject {
    fn from(child: VChild<Button>) -> Self {
        Box::new(Button {
            props: child.props,
            ..Default::default()
        })
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
}
