impl From<Icon> for char {
    fn from(icon: Icon) -> char {
        char_from_icon(icon)
    }
}

/// Material icons font (`MaterialIcons-Regular.ttf`), licensed under the Apache 2.0 license
/// While attribution is not strictly required, it is appreciated.
/// See https://github.com/google/material-design-icons for more information

#[cfg(all(feature = "outlined", not(feature = "rounded"), not(feature = "sharp")))]
pub const FONT: &[u8] = include_bytes!("../assets/MaterialSymbolsOutlined.woff2");

#[cfg(all(feature = "rounded", not(feature = "outlined"), not(feature = "sharp")))]
pub const FONT: &[u8] = include_bytes!("../assets/MaterialSymbolsRounded.woff2");

// Default
#[cfg(any(
    feature = "sharp",
    all(
        not(feature = "outlined"),
        not(feature = "rounded"),
        not(feature = "sharp")
    ),
    all(feature = "outlined", feature = "rounded"),
    all(feature = "outlined", feature = "sharp"),
    all(feature = "rounded", feature = "sharp"),
))]
pub const FONT: &[u8] = include_bytes!("../assets/MaterialSymbolsSharp.woff2");

use std::fmt;
impl fmt::Display for Icon {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", char_from_icon(*self))
    }
}
