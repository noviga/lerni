use std::rc::Rc;
use yew::prelude::*;

use crate::{
    properties::{Align, VAlign},
    widgets::{FromProperties, Widget},
};

/// Label widget.
#[derive(Clone)]
pub struct Label {
    props: Rc<Props>,
}

/// Label properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_else(|| "Label".to_string())]
    pub text: String,
    #[prop_or_default]
    pub x: i32,
    #[prop_or_default]
    pub y: i32,
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

impl Component for Label {
    type Message = ();
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        Self {
            props: Rc::new(ctx.props().clone()),
        }
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

impl Widget for Label {
    fn set_frame(&mut self, x: i32, y: i32, width: i32, height: i32) {
        let p = Rc::make_mut(&mut self.props);
        p.x = x + width / 2;
        p.y = y + height / 2;
    }

    fn render(&self) -> Html {
        let p = &self.props;
        html! {
            <Label text={ p.text.clone() } x={ p.x } y={ p.y } font_size={ p.font_size }
                align={ p.align.clone() } valign={ p.valign.clone() } />
        }
    }
}

impl FromProperties for Label {
    fn from_properties(props: Rc<Props>) -> Self {
        Self { props }
    }
}
