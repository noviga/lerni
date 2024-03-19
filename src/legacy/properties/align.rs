/// Horizontal align.
#[derive(Clone, Default, PartialEq)]
pub enum Align {
    /// Left horizontal align.
    Left,
    /// Center horizontal align.
    #[default]
    Center,
    /// Right horizontal align.
    Right,
    /// Fill all horizontal space.
    Fill,
}

/// Vertical align.
#[derive(Clone, Default, PartialEq)]
pub enum VAlign {
    /// Top vertical align.
    Top,
    /// Middle vertical align.
    #[default]
    Middle,
    /// Bottom vertical align.
    Bottom,
    /// Fill all vertical space.
    Fill,
}
