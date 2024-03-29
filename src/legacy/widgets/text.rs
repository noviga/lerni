use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};
use yew::{prelude::*, virtual_dom::VNode};

use crate::{properties::Color, widgets::Frame};

/// Text widget.
pub struct Text {
    frame: Frame,
    canvas: CanvasRenderingContext2d,
    words: Vec<String>,
    letter_counters: Vec<usize>,
    total_letters: usize,
    rects: Vec<Rect>,
    expand: i32,
    _context_listener: ContextHandle<Frame>,
}

struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
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
    pub line_height: f32,
    #[prop_or(1.4)]
    pub indent: f32,
    #[prop_or(Color::PaleGreen)]
    pub marker_color: Color,
    #[prop_or_default]
    pub words_read: usize,
    #[prop_or_default]
    pub lattice: bool,
    #[prop_or_default]
    pub erase_top: f32,
    #[prop_or_default]
    pub erase_bottom: f32,
    #[prop_or_default]
    pub onread: Callback<(usize, usize, usize)>,
}

pub enum Msg {
    ContextUpdated(Frame),
    Clicked(i32, i32),
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

        let canvas = Self::canvas_context(p);

        let expand = Self::text_width(" ", &canvas) / 2 + 1;

        let mut text = Self {
            frame,
            canvas,
            words: Default::default(),
            letter_counters: Default::default(),
            total_letters: Default::default(),
            rects: Default::default(),
            expand,
            _context_listener,
        };
        text.wrap(p);
        let letters = text.letter_counters.iter().take(p.words_read).sum();
        p.onread.emit((p.words_read, letters, text.total_letters));
        text
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        let p = ctx.props();
        match msg {
            Msg::ContextUpdated(frame) => {
                self.frame = frame;
                true
            }
            Msg::Clicked(x, y) => {
                if let Some(index) = self.find_word_index(x, y) {
                    let words_read = index + 1;
                    let letters = self.letter_counters.iter().take(words_read).sum();
                    p.onread.emit((words_read, letters, self.total_letters));
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

        let onclick = {
            let fx = f.fx;
            let fy = f.fy;
            ctx.link().callback(move |e: MouseEvent| {
                let x = (e.offset_x() as f32 * fx).round() as i32;
                let y = (e.offset_y() as f32 * fy).round() as i32;
                Msg::Clicked(x, y)
            })
        };

        let class = if p.bold { "has-text-weight-bold" } else { "" };

        let lattice = if p.lattice {
            let width = p.font_size / 2;
            let dx = 5 * width;
            let count = f.width / dx;
            html! {
                for (0..count).map(|i| html_nested!(<rect x={ (f.x + dx / 2 + i * dx).to_string() } y={ f.y.to_string() }
                    width={ width.to_string() } height={ f.height.to_string() }
                    rx={ (width / 2).to_string() }
                    stroke={ p.color.to_string() } stroke-width="6"
                    fill="white" pointer-events="none" />))
            }
        } else {
            Default::default()
        };

        let word = |(i, r): (usize, &Rect)| {
            html_nested! {
                <text x={ (r.x + r.width / 2).to_string() }
                    y={ (r.y + r.height / 2).to_string() } { class } text-anchor="middle"
                    dominant-baseline="central" font-size={ p.font_size.to_string() }
                    style={ format!("font-family: {}", p.font) } fill={ p.color.to_string() }
                    pointer-events="none">
                    { self.words[i].clone() }
                </text>
            }
        };

        let erase = |r: &Rect| {
            let erase_top = if p.erase_top > 0.0 {
                let h = (p.erase_top * r.height as f32).round() as i32;
                html_nested!(<rect fill="white" x={ r.x.to_string() } y={ r.y.to_string() }
                    width={ r.width .to_string() } height={ h.to_string() }/>)
            } else {
                Default::default()
            };
            let erase_bottom = if p.erase_bottom > 0.0 {
                let h = (p.erase_bottom * r.height as f32).round() as i32;
                html_nested!(<rect fill="white" x={ r.x.to_string() } y={ (r.y + r.height - h).to_string() }
                    width={ r.width .to_string() } height={ h.to_string() }/>)
            } else {
                Default::default()
            };
            html_nested! {
                <>
                    { erase_top }
                    { erase_bottom }
                </>
            }
        };

        html! {
            <>
                <rect x={ f.x.to_string() } y={ f.y.to_string() } width={ f.width.to_string() }
                    height={ f.height.to_string() } fill="white" { onclick } />
                { for self.rects.iter().enumerate().map(word) }
                {
                    if p.erase_top > 0.0 || p.erase_bottom > 0.0 {
                        self.rects.iter().map(erase).collect::<Html>()
                    } else {
                        Default::default()
                    }
                }
                { lattice }
                {
                    for self.rects.iter().take(p.words_read).map(|r| {
                        html! {
                            <rect x={ (r.x - self.expand).to_string() }
                                y={ (r.y - self.expand).to_string() }
                                width={ (r.width + 2 * self.expand).to_string() }
                                height={ (r.height + 2 * self.expand).to_string() }
                                rx={ self.expand.to_string() } ry={ self.expand.to_string() }
                                fill={ p.marker_color.to_string() } pointer-events="none" />
                        }
                    })
                }
                { for self.rects.iter().take(p.words_read).enumerate().map(word) }
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

    fn text_width(text: &str, canvas: &CanvasRenderingContext2d) -> i32 {
        canvas.measure_text(text).unwrap().width() as i32
    }

    fn wrap(&mut self, props: &Props) {
        let children = props.children.iter().map(|item| {
            if let VNode::VText(node) = item {
                node.text
            } else {
                Default::default()
            }
        });

        let mut y = self.frame.y;
        for child in children {
            let mut words = child.split(' ');
            let first_word = words.next().unwrap().to_string();
            self.letter_counters
                .push(first_word.chars().filter(|c| c.is_alphabetic()).count());

            let mut indent = (props.indent * props.font_size as f32).round() as i32;

            let dy = (props.line_height * props.font_size as f32).round() as i32;
            let mut x = self.frame.x + indent;
            let mut lines = Vec::new();
            let mut line = first_word.clone();
            self.rects.push(Rect {
                x,
                y,
                width: Self::text_width(&first_word, &self.canvas),
                height: props.font_size,
            });
            x += Self::text_width(&first_word, &self.canvas);
            self.words.push(first_word);
            for word in words {
                self.words.push(word.to_string());
                self.letter_counters
                    .push(word.chars().filter(|c| c.is_alphabetic()).count());
                if x + Self::text_width(&format!(" {word}"), &self.canvas)
                    > self.frame.x + self.frame.width
                {
                    lines.push(line.clone());
                    line = word.to_string();
                    indent = 0;
                    x = self.frame.x;
                    y += dy;
                } else {
                    line.push(' ');
                    x = self.frame.x + indent + Self::text_width(&line, &self.canvas);
                    line.push_str(word);
                }
                self.rects.push(Rect {
                    x,
                    y,
                    width: Self::text_width(word, &self.canvas),
                    height: props.font_size,
                });
                x = self.frame.x + indent + Self::text_width(&line, &self.canvas);
            }
            if !line.is_empty() {
                lines.push(line);
            }

            y += dy;
        }
        self.total_letters = self.letter_counters.iter().sum();
    }

    fn find_word_index(&self, x: i32, y: i32) -> Option<usize> {
        if x < self.frame.x
            || x > self.frame.x + self.frame.width
            || y < self.frame.y
            || y > self.frame.y + self.frame.height
        {
            return None;
        }
        self.rects
            .iter()
            .enumerate()
            .find(|(_, r)| x >= r.x && x <= r.x + r.width && y >= r.y && y <= r.y + r.height)
            .map(|(i, _)| i)
    }
}
