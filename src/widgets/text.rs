extern crate alloc;

use alloc::rc::Rc;
use leptos::prelude::*;
use rand::{Rng, prelude::SliceRandom, rngs::OsRng};
use wasm_bindgen::JsValue;
use web_sys::{CanvasRenderingContext2d, HtmlCanvasElement, MouseEvent};

use crate::{Color, Frame, SvgFrame, VAlign, into_strings::IntoStrings, use_frame};

#[derive(Debug, Clone)]
struct TextProperties<'a> {
    bold: bool,
    font_size: i32,
    font: &'a str,
    line_height: f32,
    indent: f32,
}

#[derive(Clone)]
struct Rect {
    pub x: i32,
    pub y: i32,
    pub width: i32,
    pub height: i32,
}

struct Output {
    pub words: Vec<String>,
    pub rects: Rc<Vec<Rect>>,
    pub letter_counters: Vec<usize>,
    pub font_size: i32,
    pub canvas: CanvasRenderingContext2d,
}

/// Text widget.
#[component]
pub fn Text<T: IntoStrings>(
    #[prop(optional)] bold: bool,
    #[prop(optional)] font_size: i32,
    #[prop(default = Color::Black)] color: Color,
    #[prop(default = "sans-serif".to_string(), into)] font: String,
    #[prop(default = 1.2)] line_height: f32,
    #[prop(default = 1.4)] indent: f32,
    #[prop(default = VAlign::Top)] valign: VAlign,
    #[prop(default = Color::PaleGreen)] marker_color: Color,
    #[prop(optional, into)] lattice: Signal<bool>,
    #[prop(optional, into)] reverse_words: Signal<bool>,
    #[prop(optional, into)] shuffle_letters: Signal<bool>,
    #[prop(optional, into)] erase_top: Signal<f32>,
    #[prop(optional, into)] erase_bottom: Signal<f32>,
    #[prop(optional, into)] words_read: RwSignal<usize>,
    #[prop(optional, into)] word_count: RwSignal<usize>,
    #[prop(optional, into)] letters_read: RwSignal<usize>,
    #[prop(optional, into)] letters_total: RwSignal<usize>,
    #[prop(optional, into)] on_click: Option<Callback<MouseEvent>>,
    #[prop(default = true.into(), into)] visible: Signal<bool>,
    #[prop(default = "all .3s".to_string(), into)] transition: String,
    children: TypedChildren<T>,
) -> impl IntoView {
    let props = TextProperties {
        bold,
        font_size,
        font: &font,
        line_height,
        indent,
    };
    let children = children.into_inner();
    let sentences = children().into_inner().into_strings();
    let f = use_frame();
    let Output {
        words,
        rects,
        letter_counters,
        font_size,
        canvas,
    } = wrap(sentences, &props, &f, &valign);

    letters_total.set(letter_counters.iter().sum());
    word_count.set(words.len());

    let get_word = move |i: usize, hidden: bool| {
        let word: &String = words.get(i).unwrap();
        if !hidden && reverse_words.get() {
            reverse_word(word)
        } else if !hidden && shuffle_letters.get() {
            shuffle_word(word)
        } else {
            word.clone()
        }
    };

    let word = {
        |i, r: &Rect, hidden: bool| {
            view! {
                <text
                    visibility=move || { (hidden && i >= words_read.get()).then_some("hidden") }
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
                    {move || get_word(i, hidden)}
                </text>
            }
        }
    };

    let erase = |r: &Rect| {
        let rx = r.x;
        let ry = r.y;
        let rh = r.height;
        let rw = r.width;
        view! {
            {move || {
                (erase_top.get() > 0.0)
                    .then(|| {
                        let h = (erase_top.get() * rh as f32).round() as i32;
                        view! {
                            <rect
                                fill="white"
                                pointer-events="none"
                                x=rx
                                y=ry
                                width=rw
                                height=h
                            ></rect>
                        }
                    })
            }}

            {move || {
                (erase_bottom.get() > 0.0)
                    .then(|| {
                        let h = (erase_bottom.get() * rh as f32).round() as i32;
                        view! {
                            <rect
                                fill="white"
                                pointer-events="none"
                                x=rx
                                y=ry + rh - h
                                width=rw
                                height=h
                            ></rect>
                        }
                    })
            }}
        }
    };

    let r = Rc::clone(&rects);
    let on_click = move |e: MouseEvent| {
        let svg: Option<SvgFrame> = use_context();
        if let Some(svg) = svg {
            let x = e.offset_x() * svg.width / svg.client_width;
            let y = e.offset_y() * svg.height / svg.client_height;
            if x >= f.x && x <= f.x + f.width && y >= f.y && y <= f.y + f.height {
                if let Some(index) = find_word_index(x, y, &r) {
                    words_read.set(index + 1);
                }
            }
        }
        if let Some(cb) = on_click {
            cb.run(e);
        }
    };

    Effect::new(move |_| letters_read.set(letter_counters[0..words_read.get()].iter().sum()));

    let expand = text_width(" ", &canvas) / 2 + 1;

    view! {
        <g
            style:opacity=move || if visible.get() { "1" } else { "0" }
            style:visibility=move || if visible.get() { "visible" } else { "hidden" }
            style:transition=transition
        >
            <rect x=f.x y=f.y width=f.width height=f.height fill="white" on:click=on_click></rect>
            {rects.iter().enumerate().map(|(i, r)| word.clone()(i, r, false)).collect_view()}
            {rects.iter().map(erase).collect_view()}

            {move || {
                lattice
                    .get()
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
                    })
            }}

            {rects
                .iter()
                .enumerate()
                .map(|(i, r)| {
                    view! {
                        <rect
                            visibility=move || { (i >= words_read.get()).then_some("hidden") }
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

            {rects.iter().enumerate().map(|(i, r)| word.clone()(i, r, true)).collect_view()}
        </g>
    }
}

fn reverse_word(word: &str) -> String {
    let mut result = String::new();
    let mut last = 0;
    // Reverse only alphabetic chars
    for (i, matched) in
        word.match_indices(&[',', '.', '!', '?', ':', ';', '…', '(', ')', '«', '»', '"'])
    {
        if last != i {
            result.push_str(&word[last..i].chars().rev().collect::<String>());
        }
        result.push_str(matched);
        last = i + matched.len();
    }
    if last < word.len() {
        result.push_str(&word[last..].chars().rev().collect::<String>());
    }
    result
}

fn shuffle_word(word: &str) -> String {
    let mut result = String::new();
    let mut last = 0;

    let shuffle = |word: &str| -> String {
        let mut rng = OsRng;
        let mut word: Vec<_> = word.chars().collect();
        let len = word.len();
        if len > 3 {
            let start = rng.gen_range(1..len / 2);
            let end = rng.gen_range((len + 1) / 2..len);
            word[start..end].shuffle(&mut rng);
        }
        word.into_iter().collect()
    };
    // Shuffle only alphabetic chars except the first and last
    for (i, matched) in word.match_indices(|c: char| !(c.is_alphanumeric())) {
        if last != i {
            result.push_str(shuffle(&word[last..i]).as_str());
        }
        result.push_str(matched);
        last = i + matched.len();
    }
    if last < word.len() {
        result.push_str(shuffle(&word[last..]).as_str());
    }
    result
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

fn text_height(sentences: &[String], props: &TextProperties, width: i32) -> i32 {
    let mut y = 0;
    let canvas = canvas_context(props);
    for sentence in sentences {
        let mut sentence_words = sentence.split(' ');
        let first_word = sentence_words.next().unwrap();
        let mut indent = (props.indent * props.font_size as f32).round() as i32;
        let dy = (props.line_height * props.font_size as f32).round() as i32;
        let mut x = indent;
        let mut lines = Vec::new();
        let mut line = first_word.to_string();
        x += text_width(first_word, &canvas);
        for word in sentence_words {
            if x + text_width(&format!(" {word}"), &canvas) > width {
                lines.push(line.clone());
                line = word.to_string();
                indent = 0;
                y += dy;
            } else {
                line.push(' ');
                line.push_str(word);
            }
            x = indent + text_width(&line, &canvas);
        }
        if !line.is_empty() {
            lines.push(line);
        }

        y += dy;
    }
    y
}

fn wrap(sentences: Vec<String>, props: &TextProperties, frame: &Frame, valign: &VAlign) -> Output {
    let mut props = props.clone();
    if props.font_size == 0 {
        loop {
            props.font_size += 1;
            if text_height(&sentences, &props, frame.width) > frame.height {
                break;
            }
        }
        props.font_size -= 1;
    }

    let canvas = canvas_context(&props);
    let mut y = frame.y;
    let mut words = Vec::new();
    let mut rects = Vec::new();
    let mut letter_counters = Vec::new();
    for sentence in sentences {
        let mut sentence_words = sentence.split(' ');
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
            width: text_width(&first_word, &canvas),
            height: props.font_size,
        });
        x += text_width(&first_word, &canvas);
        words.push(first_word);
        for word in sentence_words {
            words.push(word.to_string());
            letter_counters.push(word.chars().filter(|c| c.is_alphabetic()).count());
            if x + text_width(&format!(" {word}"), &canvas) > frame.x + frame.width {
                lines.push(line.clone());
                line = word.to_string();
                indent = 0;
                x = frame.x;
                y += dy;
            } else {
                line.push(' ');
                x = frame.x + indent + text_width(&line, &canvas);
                line.push_str(word);
            }
            rects.push(Rect {
                x,
                y,
                width: text_width(word, &canvas),
                height: props.font_size,
            });
            x = frame.x + indent + text_width(&line, &canvas);
        }
        if !line.is_empty() {
            lines.push(line);
        }

        y += dy;
    }
    let dy = match valign {
        VAlign::Middle => {
            if let Some(last_rect) = rects.last() {
                let total_height = last_rect.y + last_rect.height - frame.y;
                (frame.height - total_height) / 2
            } else {
                0
            }
        }
        VAlign::Bottom => {
            if let Some(last_rect) = rects.last() {
                let total_height = last_rect.y + last_rect.height - frame.y;
                frame.height - total_height
            } else {
                0
            }
        }
        _ => 0,
    };
    for r in &mut rects {
        r.y += dy;
    }
    Output {
        rects: Rc::new(rects),
        words,
        letter_counters,
        font_size: props.font_size,
        canvas,
    }
}

fn find_word_index(x: i32, y: i32, rects: &[Rect]) -> Option<usize> {
    rects
        .iter()
        .position(|r| x >= r.x && x <= r.x + r.width && y >= r.y && y <= r.y + r.height)
}
