use derive_more::Display;
use leptos::{Attribute, IntoAttribute};

// TODO: Define all colors

/// Helper type to work with colors.
#[derive(Clone, Copy, Default, Display, PartialEq)]
pub enum Color {
    /// No color.
    #[default]
    None,
    /// Arbitrary color with the string value.
    #[display("{_0}")]
    Value(&'static str),

    /// Alice Blue color.
    AliceBlue,
    /// Antique White color.
    AntiqueWhite,
    /// Aqua color.
    Aqua,
    /// Aquamarine color.
    Aquamarine,
    /// Azure color.
    Azure,
    /// Beige color.
    Beige,
    /// Bisque color.
    Bisque,
    /// Black color.
    Black,
    /// Blanched Almond color.
    BlanchedAlmond,
    /// Blue color.
    Blue,
    /// Blue Violet color.
    BlueViolet,
    /// Brown color.
    Brown,
    /// Burly Wood color.
    BurlyWood,
    /// Cadet Blue color.
    CadetBlue,
    /// Chartreuse color.
    Chartreuse,
    /// Chocolate color.
    Chocolate,
    /// Coral color.
    Coral,
    /// Cornflower Blue color.
    CornflowerBlue,
    /// Cornsilk color.
    Cornsilk,
    /// Crimson color.
    Crimson,
    /// Cyan color.
    Cyan,
    /// Dark Blue color.
    DarkBlue,
    /// Dark Cyan color.
    DarkCyan,
    /// Dark Goldenrod color.
    DarkGoldenrod,
    /// Dark Gray color.
    DarkGray,
    /// Dark Green color.
    DarkGreen,
    /// Dark Grey color.
    DarkGrey,
    /// Dark Khaki color.
    DarkKhaki,
    /// Dark Magenta color.
    DarkMagenta,
    /// Dark Olive Green color.
    DarkOliveGreen,
    /// Dark Orange color.
    DarkOrange,
    /// Dark Orchid color.
    DarkOrchid,
    /// Dark Red color.
    DarkRed,
    /// Dark Salmon color.
    DarkSalmon,
    /// Dark Sea Green color.
    DarkSeaGreen,
    /// Dark Slate Blue color.
    DarkSlateBlue,
    /// Dark Slate Gray color.
    DarkSlateGray,
    /// Dark Slate Grey color.
    DarkSlateGrey,
    /// Dark Turquoise color.
    DarkTurquoise,
    /// Dark Violet color.
    DarkViolet,
    /// Deep Pink color.
    DeepPink,
    /// Deep Sky Blue color.
    DeepSkyBlue,
    /// Dim Gray color.
    DimGray,
    /// Dim Grey color.
    DimGrey,
    /// Dodger Blue color.
    DodgerBlue,
    /// Firebrick color.
    Firebrick,
    /// Floral White color.
    FloralWhite,
    /// Forest Green color.
    ForestGreen,
    /// Fuchsia color.
    Fuchsia,
    /// Gainsboro color.
    Gainsboro,
    /// Ghost White color.
    GhostWhite,
    /// Gold color.
    Gold,
    /// Goldenrod color.
    Goldenrod,
    /// Gray color.
    Gray,
    /// Green color.
    Green,
    /// Green Yellow color.
    GreenYellow,
    /// Grey color.
    Grey,
    /// Honeydew color.
    Honeydew,
    /// Hot Pink color.
    HotPink,
    /// Indian Red color.
    IndianRed,
    /// Indigo color.
    Indigo,
    /// Ivory color.
    Ivory,
    /// Khaki color.
    Khaki,
    /// Lavender color.
    Lavender,
    /// Lavender Blush color.
    LavenderBlush,
    /// Lawn Green color.
    LawnGreen,
    /// Lemon Chiffon color.
    Lemonchiffon,
    /// Light Blue color.
    LightBlue,
    /// Light Coral color.
    LightCoral,
    /// Light Cyan color.
    LightCyan,
    /// Light Goldenrod Yellow color.
    LightGoldenrodYellow,
    /// Light Gray color.
    LightGray,
    /// Light Green color.
    LightGreen,
    /// Light Grey color.
    LightGrey,
    /// Light Pink color.
    LightPink,
    /// Light Salmon color.
    LightSalmon,
    /// Light Sea Green color.
    LightSeaGreen,
    /// Light Sky Blue color.
    LightSkyBlue,
    /// Light Slate Gray color.
    LightSlateGray,
    /// Light Slate Grey color.
    LightSlateGrey,
    /// Light Steel Blue color.
    LightsteelBlue,
    /// Light Yellow color.
    LightYellow,
    /// Lime color.
    Lime,
    /// Lime Green color.
    LimeGreen,
    /// Linen color.
    Linen,
    /// Magenta color.
    Magenta,
    /// Maroon color.
    Maroon,
    /// Medium Aqua Marine color.
    MediumAquamarine,
    /// Medium Blue color.
    MediumBlue,
    /// Medium Orchid color.
    MediumOrchid,
    /// Medium Purple color.
    MediumPurple,
    /// Medium Sea Green color.
    MediumSeaGreen,
    /// Medium Slate Blue color.
    MediumSlateBlue,
    /// Medium Spring Green color.
    MediumSpringGreen,
    /// Medium Turquoise color.
    MediumTurquoise,
    /// Medium Violet Red color.
    MediumVioletRed,
    /// Midnight Blue color.
    MidnightBlue,
    /// Mint Cream color.
    MintCream,
    /// Misty Rose color.
    MistyRose,
    /// Moccasin color.
    Moccasin,
    /// Navajo White color.
    NavajoWhite,
    /// Navy color.
    Navy,
    /// Old Lace color.
    OldLace,
    /// Olive color.
    Olive,
    /// Olive Drab color.
    OliveDrab,
    /// Orange color.
    Orange,
    /// Orange Red color.
    OrangeRed,
    /// Orchid color.
    Orchid,
    /// Pale Goldenrod color.
    PaleGoldenrod,
    /// Pale Green color.
    PaleGreen,
    /// Pale Turquoise color.
    PaleTurquoise,
    /// Pale Violet Red color.
    PaleVioletRed,
    /// Papaya Whip color.
    PapayaWhip,
    /// Peach Puff color.
    PeachPuff,
    /// Peru color.
    Peru,
    /// Pink color.
    Pink,
    /// Plum color.
    Plum,
    /// Powder Blue color.
    PowderBlue,
    /// Purple color.
    Purple,
    /// Red color.
    Red,
    /// Rosy Brown color.
    RosyBrown,
    /// Royal Blue color.
    RoyalBlue,
    /// Saddle Brown color.
    SaddleBrown,
    /// Salmon color.
    Salmon,
    /// Sandy Brown color.
    SandyBrown,
    /// Sea Green color.
    SeaGreen,
    /// Seashell color.
    Seashell,
    /// Sienna color.
    Sienna,
    /// Silver color.
    Silver,
    /// Sky Blue color.
    SkyBlue,
    /// Slate Blue color.
    SlateBlue,
    /// Slate Gray color.
    SlateGray,
    /// Slate Grey color.
    SlateGrey,
    /// Snow color.
    Snow,
    /// Spring Green color.
    SpringGreen,
    /// Steel Blue color.
    SteelBlue,
    /// Tan color.
    Tan,
    /// Teal color.
    Teal,
    /// Thistle color.
    Thistle,
    /// Tomato color.
    Tomato,
    /// Transparent color.
    Transparent,
    /// Turquoise color.
    Turquoise,
    /// Violet color.
    Violet,
    /// Wheat color.
    Wheat,
    /// White color.
    White,
    /// White Smoke color.
    WhiteSmoke,
    /// Yellow color.
    Yellow,
    /// Yellow Green color.
    YellowGreen,
}

impl IntoAttribute for Color {
    fn into_attribute(self) -> Attribute {
        Attribute::String(self.to_string().into())
    }

    fn into_attribute_boxed(self: Box<Self>) -> leptos::Attribute {
        self.into_attribute()
    }
}
