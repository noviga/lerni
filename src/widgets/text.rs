use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{properties::Color, widgets::Frame};

/// Text properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub bold: bool,
    /// Font size (default: 48px).
    #[prop_or(48)]
    pub font_size: i32,
    #[prop_or(Color::Black)]
    pub color: Color,
    #[prop_or_else(|| "sans-serif".to_string())]
    pub font: String,
}

/// Text widget.
#[function_component]
pub fn Text(props: &Props) -> Html {
    let f = use_context::<Frame>().unwrap();

    let x = f.x.to_string();
    let y = f.y.to_string();

    let class = classes!(props.bold.then_some("has-text-weight-bold"));

    let text = props.children.iter().map(|item| {
        if let VNode::VText(node) = item {
            node.text
        } else {
            Default::default()
        }
    });

    let context = canvas_context(props);

    html! {
        <text { x } { y } { class } font-size={ props.font_size.to_string() }
            style={ format!("font-family:{}", props.font) } fill={ props.color.to_string() }
            pointer-events="none">
            { for text.map(|ref line| wrap(line, &f, 2 * props.font_size, &context)) }
        </text>
    }
}

fn canvas_context(props: &Props) -> CanvasRenderingContext2d {
    let doc = web_sys::window()
        .and_then(|win| win.document())
        .expect("Unable to get document");

    let canvas = HtmlCanvasElement::from(JsValue::from(doc.create_element("canvas").unwrap()));
    let context =
        CanvasRenderingContext2d::from(JsValue::from(canvas.get_context("2d").unwrap().unwrap()));

    let font_weight = if props.bold { 700 } else { 400 };
    context.set_font(&format!(
        "{font_weight} {}px {}",
        props.font_size, props.font
    ));
    context
}

fn text_width(text: &str, context: &CanvasRenderingContext2d) -> i32 {
    context.measure_text(text).unwrap().width() as i32
}

fn wrap(text: &str, frame: &Frame, indent: i32, context: &CanvasRenderingContext2d) -> Html {
    let mut words = text.split(' ');
    let mut lines = Vec::new();
    let mut line = words.next().unwrap().to_string();
    let mut delta = indent;

    for word in words {
        if text_width(&format!("{line} {word}"), context) + delta > frame.width {
            // TODO: Find better solution to move string contents
            lines.push(line.clone());
            line = word.to_string();
            delta = 0;
        } else {
            line.push(' ');
            line.push_str(word);
        }
    }
    if !line.is_empty() {
        lines.push(line);
    }
    let mut indent = indent;
    lines
        .into_iter()
        .map(|line| {
            let res =
                html!(<tspan x={ (frame.x + indent).to_string() } dy="1.2em">{ line }</tspan>);
            indent = 0;
            res
        })
        .collect()
}
