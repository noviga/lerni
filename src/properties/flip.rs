/// Flip is an enum that represents the flip state of an object.
#[derive(Clone, Default, PartialEq)]
pub enum Flip {
    /// No flip.
    #[default]
    None,
    /// Horizontal flip.
    Horizontal,
    /// Vertical flip.
    Vertical,
}
