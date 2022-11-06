/// Main SVG width in pixels.
pub const SVG_WIDTH: usize = 640;
/// Main SVG height in pixels.
pub const SVG_HEIGHT: usize = 360;

/// Horizontal align.
#[derive(Clone, PartialEq)]
pub enum Align {
    /// Left horizontal align.
    Left,
    /// Center horizontal align.
    Center,
    /// Right horizontal align.
    Right,
}

/// Vertical align.
#[derive(Clone, PartialEq)]
pub enum VAlign {
    /// Top vertical align.
    Top,
    /// Middle vertical align.
    Middle,
    /// Bottom vertical align.
    Bottom,
}
