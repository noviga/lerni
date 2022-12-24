use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{debug, properties::Color, widgets::Frame};

/// Text widget.
pub struct Text {
    frame: Frame,
    lines: Vec<Html>,
    letter_counters: Vec<usize>,
    total_letters: usize,
    rects: Vec<Frame>,
    expand: i32,
    index: usize,
    _context_listener: ContextHandle<Frame>,
}

/// Text properties.
#[derive(Clone, Default, Properties, PartialEq)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or(48)]
    pub font_size: i32,
    #[prop_or(Color::Black)]
    pub color: Color,
    #[prop_or_else(|| "sans-serif".to_string())]
    pub font: String,
    #[prop_or(1.2)]
    pub line_height: f64,
    #[prop_or(1.4)]
    pub indent: f64,
    #[prop_or_default]
    pub marker_x: i32,
    #[prop_or_default]
    pub marker_y: i32,
    #[prop_or(Color::PaleGreen)]
    pub marker_color: Color,
    #[prop_or_default]
    pub onread: Callback<(usize, usize)>,
}

pub enum Msg {
    ContextUpdated(Frame),
    MarkerIndexChanged(usize),
}

impl Component for Text {
    type Message = Msg;
    type Properties = Props;

    fn create(ctx: &Context<Self>) -> Self {
        let p = ctx.props();
        let (frame, _context_listener) = ctx
            .link()
            .context(ctx.link().callback(Msg::ContextUpdated))
            .expect("No context provided");

        let context = Self::canvas_context(p);
        let mut letter_counters = Vec::new();
        let mut rects = Vec::new();
        let lines: Vec<_> = p
            .children
            .iter()
            .map(|item| {
                let text = if let VNode::VText(node) = item {
                    node.text
                } else {
                    Default::default()
                };
                Self::wrap(&text, &frame, p, &context, &mut letter_counters, &mut rects)
            })
            .collect();

        let total_letters = letter_counters.iter().fold(0, |sum, letters| sum + letters);

        let expand = Self::text_width(" ", &context) / 2 + 1;

        Self {
            frame,
            lines,
            letter_counters,
            total_letters,
            rects,
            expand,
            index: 0,
            _context_listener,
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let p = ctx.props();
        match msg {
            Msg::ContextUpdated(frame) => {
                self.frame = frame;
                true
            }
            Msg::MarkerIndexChanged(index) => {
                if self.index != index {
                    let letters = self
                        .letter_counters
                        .iter()
                        .take(index + 1)
                        .fold(0, |sum, letters| sum + letters);
                    debug!("Read: ({}/{})", letters, self.total_letters);
                    self.index = index;
                    p.onread.emit((letters, self.total_letters));
                    true
                } else {
                    false
                }
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let p = ctx.props();
        let f = &self.frame;

        let index = self.marker_index(p);
        if let Some(index) = index {
            if self.index != index {
                ctx.link()
                    .callback(|i| Msg::MarkerIndexChanged(i))
                    .emit(index);
            }
        }

        let x = f.x.to_string();
        let y = (f.y - (p.line_height * p.font_size as f64 / 2.0).round() as i32).to_string();

        let class = classes!(p.bold.then_some("has-text-weight-bold"));

        html! {
            <>
                {
                    if let Some(index) = index {
                        self.rects.iter().take(index + 1).map(|r| {
                            html! {
                                <rect x={ (r.x - self.expand).to_string() }
                                    y={ (r.y - self.expand).to_string() }
                                    width={ (r.width + 2 * self.expand).to_string() }
                                    height={ (r.height + 2 * self.expand).to_string() }
                                    rx={ self.expand.to_string() } ry={ self.expand.to_string() }
                                    fill={ p.marker_color.to_string() } />
                            }
                        }).collect()
                    } else {
                        html!()
                    }
                }
                <text { x } { y } { class } font-size={ p.font_size.to_string() }
                    style={ format!("font-family:{}", p.font) } fill={ p.color.to_string() }
                    pointer-events="none">
                    { for self.lines.iter().cloned() }
                </text>
            </>
        }
    }
}

impl Text {
    fn canvas_context(props: &Props) -> CanvasRenderingContext2d {
        let doc = web_sys::window()
            .and_then(|win| win.document())
            .expect("Unable to get document");

        let canvas = HtmlCanvasElement::from(JsValue::from(doc.create_element("canvas").unwrap()));
        let context = CanvasRenderingContext2d::from(JsValue::from(
            canvas.get_context("2d").unwrap().unwrap(),
        ));

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

    fn wrap(
        text: &str,
        frame: &Frame,
        props: &Props,
        context: &CanvasRenderingContext2d,
        letter_counters: &mut Vec<usize>,
        rects: &mut Vec<Frame>,
    ) -> Html {
        let mut words = text.split(' ');
        let first_word = words.next().unwrap().to_string();
        letter_counters.push(first_word.chars().filter(|c| c.is_alphabetic()).count());

        let indent = (props.indent * props.font_size as f64).round() as i32;

        let dy = (props.line_height * props.font_size as f64).round() as i32;
        let mut x = frame.x + indent;
        let mut y = rects.last().map(|r| r.y + dy).unwrap_or(frame.y);
        let mut lines = Vec::new();
        let mut line = first_word.clone();
        rects.push(Frame {
            x,
            y,
            width: Self::text_width(&first_word, context),
            height: props.font_size,
        });
        x += Self::text_width(&format!(" {first_word}"), context);
        for word in words {
            letter_counters.push(word.chars().filter(|c| c.is_alphabetic()).count());
            if x + Self::text_width(&format!(" {word}"), context) > frame.x + frame.width {
                lines.push(line.clone());
                line = word.to_string();
                x = frame.x;
                y += dy;
            } else {
                line.push(' ');
                line.push_str(word);
            }
            rects.push(Frame {
                x,
                y,
                width: Self::text_width(word, context),
                height: props.font_size,
            });
            x += Self::text_width(&format!(" {word}"), context);
        }
        if !line.is_empty() {
            lines.push(line);
        }

        let mut indent = indent;
        lines
            .into_iter()
            .map(|line| {
                let res = html! {
                    <tspan x={ (frame.x + indent).to_string() }
                        dy={ dy.to_string() } dominant-baseline="middle">
                        { line }
                    </tspan>
                };
                indent = 0;
                res
            })
            .collect()
    }

    fn marker_index(&self, props: &Props) -> Option<usize> {
        if props.marker_x < self.frame.x
            || props.marker_x > self.frame.x + self.frame.width
            || props.marker_y < self.frame.y
            || props.marker_y > self.frame.y + self.frame.height
        {
            return None;
        }
        self.rects
            .iter()
            .enumerate()
            .find(|(_, r)| {
                props.marker_x >= r.x
                    && props.marker_x <= r.x + r.width
                    && props.marker_y >= r.y
                    && props.marker_y <= r.y + r.height
            })
            .map(|(i, _)| i)
    }
}
