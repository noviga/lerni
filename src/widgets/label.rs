use yew::prelude::*;

use crate::{
    properties::{Align, VAlign},
    widgets::Widget,
};

/// Label widget.
pub struct Label {
    props: Props,
}

/// Label properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Label".to_string())]
    pub text: String,
    pub x: usize,
    pub y: usize,
    /// Font size (default: 48px).
    #[prop_or(48)]
    pub font_size: usize,
    /// Horizontal align (default: Center).
    #[prop_or(Align::Center)]
    pub align: Align,
    /// Vertical align (default: Middle).
    #[prop_or(VAlign::Middle)]
    pub valign: VAlign,
}

impl Widget for Label {
    fn render(&self, x: usize, y: usize, width: usize, height: usize) -> Html {
        let x = x + width / 2;
        let y = y + height / 2;
        html! {
            <Label text={ self.props.text.clone() } { x } { y } />
        }
    }
}

impl Component for Label {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: ctx.props().clone(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, _msg: Self::Message) -> bool {
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let anchor = match p.align {
            Align::Left => "start",
            Align::Center => "middle",
            Align::Right => "end",
        };
        let baseline = match p.valign {
            VAlign::Top => "hanging",
            VAlign::Middle => "central",
            VAlign::Bottom => "text-top",
        };
        let x = p.x.to_string();
        let y = p.y.to_string();

        html! {
            <text { x } { y } font-size={ p.font_size.to_string() } text-anchor={ anchor }
                dominant-baseline={ baseline } pointer-events="none">
                { &p.text }
            </text>
        }
    }
}

/// Creates a new `Button` widget.
pub fn label(text: &str) -> Box<Label> {
    Box::new(Label {
        props: Props {
            text: text.to_string(),
            ..Default::default()
        },
    })
}
