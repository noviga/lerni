pub const SVG_WIDTH: usize = 640;
pub const SVG_HEIGHT: usize = 360;

#[derive(Clone, PartialEq)]
pub enum Align {
    Left,
    Center,
    Right,
}

#[derive(Clone, PartialEq)]
pub enum VAlign {
    Top,
    Middle,
    Bottom,
}
