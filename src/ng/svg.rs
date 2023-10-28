use leptos::*;

use crate::ng::{use_frame, Align, Frame, VAlign};

/// SVG widget.
#[component]
pub fn Svg(
    width: i32,
    height: i32,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = 1.0)] scale: f32,
    #[prop(optional)] flip_x: bool,
    #[prop(optional)] flip_y: bool,
    children: Children,
) -> impl IntoView {
    let f = use_frame();
    let transform = calc_transform(&f, width, height, align, valign, scale, flip_x, flip_y);

    view! { cx, <g transform={transform}>{children(cx)}</g> }
}

/// SVG-from-file widget.
#[component]
pub fn SvgFile(
    cx: Scope,
    width: i32,
    height: i32,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = 1.0)] scale: f32,
    #[prop(optional)] flip_x: bool,
    #[prop(optional)] flip_y: bool,
    src: &'static str,
) -> impl IntoView {
    let f = use_frame(cx);
    let transform = calc_transform(&f, width, height, align, valign, scale, flip_x, flip_y);

    view! { cx, <g transform={transform} inner_html={src}/> }
}

fn calc_transform(
    f: &Frame,
    width: i32,
    height: i32,
    align: Align,
    valign: VAlign,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
) -> String {
    let scale = if matches!(align, Align::Fill) || matches!(valign, VAlign::Fill) {
        let sx = f.width as f32 / width as f32;
        let sy = f.height as f32 / height as f32;
        sx.min(sy)
    } else {
        scale
    };

    let width = (scale * width as f32).round() as i32;
    let height = (scale * height as f32).round() as i32;

    let mut x = match align {
        Align::Left => f.x,
        Align::Center | Align::Fill => f.x + (f.width - width) / 2,
        Align::Right => f.x + f.width - width,
    };
    let mut y = match valign {
        VAlign::Top => f.y,
        VAlign::Middle | VAlign::Fill => f.y + (f.height - height) / 2,
        VAlign::Bottom => f.y + f.height - height,
    };

    let mut sx = scale;
    let mut sy = scale;

    if flip_x {
        sx = -sx;
        x += width;
    }
    if flip_y {
        sy = -sy;
        y += height;
    }
    view! { <g transform=format!("translate({x} {y}) scale({sx} {sy})")>{children()}</g> }
}
