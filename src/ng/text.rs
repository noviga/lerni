use leptos::*;
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement};

use crate::ng::{use_frame, Color, Frame};

struct TextProperties<'a> {
    bold: bool,
    font_size: i32,
    font: &'a str,
    line_height: f32,
    indent: f32,
}

struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

struct Output {
    pub words: Vec<String>,
    pub rects: Vec<Rect>,
    pub letter_counters: Vec<usize>,
}

/// Text widget.
#[component]
pub fn Text(
    #[prop(optional)] bold: bool,
    #[prop(default = 48)] font_size: i32,
    #[prop(default = Color::Black)] color: Color,
    #[prop(default = "sans-serif".to_string(), into)] font: String,
    #[prop(default = 1.2)] line_height: f32,
    #[prop(default = 1.4)] indent: f32,
    #[prop(default = Color::PaleGreen)] marker_color: Color,
    #[prop(optional)] words_read: usize,
    #[prop(optional)] lattice: bool,
    #[prop(optional)] erase_top: f32,
    #[prop(optional)] erase_bottom: f32,
    children: Children,
) -> impl IntoView {
    let props = TextProperties {
        bold,
        font_size,
        font: &font,
        line_height,
        indent,
    };
    let canvas = canvas_context(&props);
    let children = children().nodes;
    let f = use_frame();
    let Output {
        words,
        rects,
        letter_counters,
    } = wrap(&children, &canvas, &props, &f);

    let _total_letters: usize = letter_counters.iter().sum();

    let word = |(i, r): (usize, &Rect)| {
        view! {
            <text
                x=r.x + r.width / 2
                y=r.y + r.height / 2
                class:has-text-weight-bold=bold
                text-anchor="middle"
                dominant-baseline="central"
                font-size=font_size
                style=format!("font-family: {}", font)
                fill=color
                pointer-events="none"
            >
                {&words[i]}
            </text>
        }
    };

    let erase = |r: &Rect| {
        view! {
            {(erase_top > 0.0)
                .then(|| {
                    let h = (erase_top * r.height as f32).round() as i32;
                    view! { <rect fill="white" x=r.x y=r.y width=r.width height=h></rect> }
                })}

            {(erase_bottom > 0.0)
                .then(|| {
                    let h = (erase_bottom * r.height as f32).round() as i32;
                    view! {
                        <rect fill="white" x=r.x y=r.y + r.height - h width=r.width height=h></rect>
                    }
                })}
        }
    };

    let expand = text_width(" ", &canvas) / 2 + 1;

    view! {
        {rects.iter().enumerate().map(word).collect_view()}
        {(erase_top > 0.0 || erase_bottom > 0.0)
            .then(|| { rects.iter().map(erase).collect_view() })}

        {lattice
            .then(|| {
                let width = font_size / 2;
                let dx = 5 * width;
                let count = f.width / dx;
                (0..count)
                    .map(|i| {
                        view! {
                            <rect
                                x=f.x + dx / 2 + i * dx
                                y=f.y
                                width=width
                                height=f.height
                                rx=width / 2
                                stroke=color
                                stroke-width="6"
                                fill="white"
                                pointer-events="none"
                            ></rect>
                        }
                    })
                    .collect_view()
            })}

        {rects
            .iter()
            .take(words_read)
            .map(|r| {
                view! {
                    <rect
                        x=r.x - expand
                        y=r.y - expand
                        width=r.width + 2 * expand
                        height=r.height + 2 * expand
                        rx=expand
                        ry=expand
                        fill=marker_color
                        pointer-events="none"
                    ></rect>
                }
            })
            .collect_view()}

        {rects.iter().take(words_read).enumerate().map(word).collect_view()}
    }
}

fn canvas_context(props: &TextProperties) -> CanvasRenderingContext2d {
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

fn text_width(text: &str, canvas: &CanvasRenderingContext2d) -> i32 {
    canvas.measure_text(text).unwrap().width() as i32
}

fn wrap(
    children: &[View],
    canvas: &CanvasRenderingContext2d,
    props: &TextProperties,
    frame: &Frame,
) -> Output {
    let children = children.iter().map(|item| {
        if let View::Text(text) = item {
            text.content.clone()
        } else {
            Default::default()
        }
    });

    let mut y = frame.y;
    let mut words = Vec::new();
    let mut rects = Vec::new();
    let mut letter_counters = Vec::new();
    for child in children {
        let mut sentence_words = child.split(' ');
        let first_word = sentence_words.next().unwrap().to_string();
        letter_counters.push(first_word.chars().filter(|c| c.is_alphabetic()).count());

        let mut indent = (props.indent * props.font_size as f32).round() as i32;

        let dy = (props.line_height * props.font_size as f32).round() as i32;
        let mut x = frame.x + indent;
        let mut lines = Vec::new();
        let mut line = first_word.clone();
        rects.push(Rect {
            x,
            y,
            width: text_width(&first_word, canvas),
            height: props.font_size,
        });
        x += text_width(&first_word, canvas);
        words.push(first_word);
        for word in sentence_words {
            words.push(word.to_string());
            letter_counters.push(word.chars().filter(|c| c.is_alphabetic()).count());
            if x + text_width(&format!(" {word}"), canvas) > frame.x + frame.width {
                lines.push(line.clone());
                line = word.to_string();
                indent = 0;
                x = frame.x;
                y += dy;
            } else {
                line.push(' ');
                x = frame.x + indent + text_width(&line, canvas);
                line.push_str(word);
            }
            rects.push(Rect {
                x,
                y,
                width: text_width(word, canvas),
                height: props.font_size,
            });
            x = frame.x + indent + text_width(&line, canvas);
        }
        if !line.is_empty() {
            lines.push(line);
        }

        y += dy;
    }
    Output {
        rects,
        words,
        letter_counters,
    }
}
