use leptos::*;

use crate::{use_frame, Align, Frame, VAlign};

struct SvgProperties {
    width: i32,
    height: i32,
    align: Align,
    valign: VAlign,
    scale: f32,
    flip_x: bool,
    flip_y: bool,
}

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
    let props = SvgProperties {
        width,
        height,
        align,
        valign,
        scale,
        flip_x,
        flip_y,
    };
    let transform = calc_transform(&f, &props);

    view! { <g transform=transform>{children()}</g> }
}

/// SVG-from-file widget.
#[component]
pub fn SvgFile(
    width: i32,
    height: i32,
    #[prop(default = Align::Center)] align: Align,
    #[prop(default = VAlign::Middle)] valign: VAlign,
    #[prop(default = 1.0)] scale: f32,
    #[prop(optional)] flip_x: bool,
    #[prop(optional)] flip_y: bool,
    src: &'static str,
) -> impl IntoView {
    let f = use_frame();
    let props = SvgProperties {
        width,
        height,
        align,
        valign,
        scale,
        flip_x,
        flip_y,
    };
    let transform = calc_transform(&f, &props);

    view! { <g transform=transform inner_html=src></g> }
}

fn calc_transform(f: &Frame, props: &SvgProperties) -> String {
    let scale = if matches!(props.align, Align::Fill) || matches!(props.valign, VAlign::Fill) {
        let sx = f.width as f32 / props.width as f32;
        let sy = f.height as f32 / props.height as f32;
        sx.min(sy)
    } else {
        props.scale
    };

    let width = (scale * props.width as f32).round() as i32;
    let height = (scale * props.height as f32).round() as i32;

    let mut x = match props.align {
        Align::Left => f.x,
        Align::Center | Align::Fill => f.x + (f.width - width) / 2,
        Align::Right => f.x + f.width - width,
    };
    let mut y = match props.valign {
        VAlign::Top => f.y,
        VAlign::Middle | VAlign::Fill => f.y + (f.height - height) / 2,
        VAlign::Bottom => f.y + f.height - height,
    };

    let mut sx = scale;
    let mut sy = scale;

    if props.flip_x {
        sx = -sx;
        x += width;
    }
    if props.flip_y {
        sy = -sy;
        y += height;
    }
    format!("translate({x} {y}) scale({sx} {sy})")
}
