use std::convert::AsRef;
use std::str::FromStr;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

/// Enum with the different colors to color your test and terminal.
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum ColorType {
    // This resets the color.
    Reset,

    Black,
    DarkGrey,

    Red,
    DarkRed,

    Green,
    DarkGreen,

    Yellow,
    DarkYellow,

    Blue,
    DarkBlue,

    Magenta,
    DarkMagenta,

    Cyan,
    DarkCyan,

    White,
    Grey,
    /// Color representing RGB-colors;
    /// r = red
    /// g = green
    /// b = blue
    Rgb {
        r: u8,
        g: u8,
        b: u8,
    },
    AnsiValue(u8),
}

impl FromStr for ColorType {
    type Err = ();

    /// Creates a `Color` from the string representation.
    ///
    /// # Remarks
    ///
    /// * `ColorType::White` is returned in case of an unknown color.
    /// * This function does not return `Err` and you can safely unwrap.
    fn from_str(src: &str) -> ::std::result::Result<Self, Self::Err> {
        let src = src.to_lowercase();

        match src.as_ref() {
            "black" => Ok(ColorType::Black),
            "dark_grey" => Ok(ColorType::DarkGrey),
            "red" => Ok(ColorType::Red),
            "dark_red" => Ok(ColorType::DarkRed),
            "green" => Ok(ColorType::Green),
            "dark_green" => Ok(ColorType::DarkGreen),
            "yellow" => Ok(ColorType::Yellow),
            "dark_yellow" => Ok(ColorType::DarkYellow),
            "blue" => Ok(ColorType::Blue),
            "dark_blue" => Ok(ColorType::DarkBlue),
            "magenta" => Ok(ColorType::Magenta),
            "dark_magenta" => Ok(ColorType::DarkMagenta),
            "cyan" => Ok(ColorType::Cyan),
            "dark_cyan" => Ok(ColorType::DarkCyan),
            "white" => Ok(ColorType::White),
            "grey" => Ok(ColorType::Grey),
            _ => Ok(ColorType::White),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::ColorType;

    #[test]
    fn test_known_color_conversion() {
        assert_eq!("black".parse(), Ok(ColorType::Black));
        assert_eq!("dark_grey".parse(), Ok(ColorType::DarkGrey));
        assert_eq!("red".parse(), Ok(ColorType::Red));
        assert_eq!("dark_red".parse(), Ok(ColorType::DarkRed));
        assert_eq!("green".parse(), Ok(ColorType::Green));
        assert_eq!("dark_green".parse(), Ok(ColorType::DarkGreen));
        assert_eq!("yellow".parse(), Ok(ColorType::Yellow));
        assert_eq!("dark_yellow".parse(), Ok(ColorType::DarkYellow));
        assert_eq!("blue".parse(), Ok(ColorType::Blue));
        assert_eq!("dark_blue".parse(), Ok(ColorType::DarkBlue));
        assert_eq!("magenta".parse(), Ok(ColorType::Magenta));
        assert_eq!("dark_magenta".parse(), Ok(ColorType::DarkMagenta));
        assert_eq!("cyan".parse(), Ok(ColorType::Cyan));
        assert_eq!("dark_cyan".parse(), Ok(ColorType::DarkCyan));
        assert_eq!("white".parse(), Ok(ColorType::White));
        assert_eq!("grey".parse(), Ok(ColorType::Grey));
    }

    #[test]
    fn test_unknown_color_conversion_yields_white() {
        assert_eq!("foo".parse(), Ok(ColorType::White));
    }
}
