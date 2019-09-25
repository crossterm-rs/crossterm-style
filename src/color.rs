//! A module that contains all the actions related to the styling of the terminal.
//! Like applying attributes to text and changing the foreground and background.

pub(crate) mod ansi;
#[cfg(windows)]
pub(crate) mod winapi;

use super::ColorType;
use crossterm_utils::Result;

/// This trait defines the actions that can be performed with terminal colors.
/// This trait can be implemented so that a concrete implementation of the ITerminalColor can fulfill
/// the wishes to work on a specific platform.
///
/// ## For example:
///
/// This trait is implemented for `WinApi` (Windows specific) and `ANSI` (Unix specific),
/// so that color-related actions can be performed on both UNIX and Windows systems.
pub(crate) trait Color : Sync + Send {
    /// Set the foreground color to the given color.
    fn set_fg(&self, fg_color: ColorType) -> Result<()>;
    /// Set the background color to the given color.
    fn set_bg(&self, fg_color: ColorType) -> Result<()>;
    /// Reset the terminal color to default.
    fn reset(&self) -> Result<()>;
}
