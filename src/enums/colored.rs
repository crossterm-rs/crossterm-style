use std::fmt::Display;

#[cfg(feature = "serde")]
use serde::{Deserialize, Serialize};

use crate::color;
use crate::enums::ColorType;

/// Can be used to easily change the front and back ground color
///
/// # Example
///
/// `Colored` implements `Display` therefore you can use it in any `write` operation.
///
/// ```no_run
/// use crossterm_style::{Colored, ColorType};
/// println!("{} Red foreground color", Colored::Fg(ColorType::Red));
/// println!("{} Blue background color", Colored::Bg(ColorType::Blue));
/// ```
#[cfg_attr(feature = "serde", derive(Serialize, Deserialize))]
#[derive(Copy, Clone, Debug, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub enum Colored {
    /// Use this if you want to change the foreground color
    Fg(ColorType),
    /// Use this if you want to change the background color
    Bg(ColorType),
}

impl Display for Colored {
    fn fmt(&self, _f: &mut ::std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> {
        let colored_terminal = color();

        match *self {
            Colored::Fg(color) => colored_terminal
                .set_fg(color)
                .map_err(|_| std::fmt::Error)?,
            Colored::Bg(color) => colored_terminal
                .set_bg(color)
                .map_err(|_| std::fmt::Error)?,
        }

        Ok(())
    }
}
