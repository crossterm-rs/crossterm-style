//! This is a `WinApi` specific implementation for styling related action.
//! This module is used for non supporting `ANSI` Windows terminals.

use std::sync::Once;

use winapi::um::wincon;

use crossterm_utils::Result;
use crossterm_winapi::{Console, Handle, HandleType, ScreenBuffer};

use crate::{Color, ColorType, Colored};

const FG_GREEN: u16 = wincon::FOREGROUND_GREEN;
const FG_RED: u16 = wincon::FOREGROUND_RED;
const FG_BLUE: u16 = wincon::FOREGROUND_BLUE;
const FG_INTENSITY: u16 = wincon::FOREGROUND_INTENSITY;

const BG_GREEN: u16 = wincon::BACKGROUND_GREEN;
const BG_RED: u16 = wincon::BACKGROUND_RED;
const BG_BLUE: u16 = wincon::BACKGROUND_BLUE;
const BG_INTENSITY: u16 = wincon::BACKGROUND_INTENSITY;

/// This struct is a WinApi implementation for color related actions.
pub(crate) struct WinApiColor;

impl WinApiColor {
    pub fn new() -> Box<WinApiColor> {
        Box::from(WinApiColor)
    }
}

impl Color for WinApiColor {
    fn set_fg(&self, fg_color: ColorType) -> Result<()> {
        // init the original color in case it is not set.
        init_console_color()?;

        let color_value = color_value(Colored::Fg(fg_color));

        let screen_buffer = ScreenBuffer::current()?;
        let csbi = screen_buffer.info()?;

        // Notice that the color values are stored in wAttribute.
        // So we need to use bitwise operators to check if the values exists or to get current console colors.
        let mut color: u16;
        let attrs = csbi.attributes();
        let bg_color = attrs & 0x0070;
        color = color_value | bg_color;

        // background intensity is a separate value in attrs,
        // wee need to check if this was applied to the current bg color.
        if (attrs & wincon::BACKGROUND_INTENSITY as u16) != 0 {
            color = color | wincon::BACKGROUND_INTENSITY as u16;
        }

        Console::from(**screen_buffer.handle()).set_text_attribute(color)?;

        Ok(())
    }

    fn set_bg(&self, bg_color: ColorType) -> Result<()> {
        // init the original color in case it is not set.
        init_console_color()?;

        let color_value = color_value(Colored::Bg(bg_color));

        let screen_buffer = ScreenBuffer::current()?;
        let csbi = screen_buffer.info()?;

        // Notice that the color values are stored in wAttribute.
        // So wee need to use bitwise operators to check if the values exists or to get current console colors.
        let mut color: u16;
        let attrs = csbi.attributes();
        let fg_color = attrs & 0x0007;
        color = fg_color | color_value;

        // Foreground intensity is a separate value in attrs,
        // So we need to check if this was applied to the current fg color.
        if (attrs & wincon::FOREGROUND_INTENSITY as u16) != 0 {
            color = color | wincon::FOREGROUND_INTENSITY as u16;
        }

        Console::from(**screen_buffer.handle()).set_text_attribute(color)?;

        Ok(())
    }

    fn reset(&self) -> Result<()> {
        // init the original color in case it is not set.
        let original_color = original_console_color();
        Console::from(Handle::new(HandleType::CurrentOutputHandle)?)
            .set_text_attribute(original_color)?;

        Ok(())
    }
}

/// This will get the winapi color value from the Color and ColorType struct
fn color_value(color: Colored) -> u16 {
    let winapi_color: u16;

    match color {
        Colored::Fg(color) => {
            winapi_color = match color {
                ColorType::Black => 0,
                ColorType::DarkGrey => FG_INTENSITY,
                ColorType::Red => FG_INTENSITY | FG_RED,
                ColorType::DarkRed => FG_RED,
                ColorType::Green => FG_INTENSITY | FG_GREEN,
                ColorType::DarkGreen => FG_GREEN,
                ColorType::Yellow => FG_INTENSITY | FG_GREEN | FG_RED,
                ColorType::DarkYellow => FG_GREEN | FG_RED,
                ColorType::Blue => FG_INTENSITY | FG_BLUE,
                ColorType::DarkBlue => FG_BLUE,
                ColorType::Magenta => FG_INTENSITY | FG_RED | FG_BLUE,
                ColorType::DarkMagenta => FG_RED | FG_BLUE,
                ColorType::Cyan => FG_INTENSITY | FG_GREEN | FG_BLUE,
                ColorType::DarkCyan => FG_GREEN | FG_BLUE,
                ColorType::White => FG_RED | FG_GREEN | FG_BLUE,
                ColorType::Grey => FG_INTENSITY | FG_RED | FG_GREEN | FG_BLUE,

                ColorType::Reset => {
                    // init the original color in case it is not set.
                    let mut original_color = original_console_color();

                    const REMOVE_BG_MASK: u16 = BG_INTENSITY | BG_RED | BG_GREEN | BG_BLUE;
                    // remove all background values from the original color, we don't want to reset those.
                    original_color &= !(REMOVE_BG_MASK);

                    original_color
                }

                /* WinApi will be used for systems that do not support ANSI, those are windows version less then 10. RGB and 255 (AnsiBValue) colors are not supported in that case.*/
                ColorType::Rgb { r: _, g: _, b: _ } => 0,
                ColorType::AnsiValue(_val) => 0,
            };
        }
        Colored::Bg(color) => {
            winapi_color = match color {
                ColorType::Black => 0,
                ColorType::DarkGrey => BG_INTENSITY,
                ColorType::Red => BG_INTENSITY | BG_RED,
                ColorType::DarkRed => BG_RED,
                ColorType::Green => BG_INTENSITY | BG_GREEN,
                ColorType::DarkGreen => BG_GREEN,
                ColorType::Yellow => BG_INTENSITY | BG_GREEN | BG_RED,
                ColorType::DarkYellow => BG_GREEN | BG_RED,
                ColorType::Blue => BG_INTENSITY | BG_BLUE,
                ColorType::DarkBlue => BG_BLUE,
                ColorType::Magenta => BG_INTENSITY | BG_RED | BG_BLUE,
                ColorType::DarkMagenta => BG_RED | BG_BLUE,
                ColorType::Cyan => BG_INTENSITY | BG_GREEN | BG_BLUE,
                ColorType::DarkCyan => BG_GREEN | BG_BLUE,
                ColorType::White => BG_INTENSITY | BG_RED | BG_GREEN | BG_BLUE,
                ColorType::Grey => BG_RED | BG_GREEN | BG_BLUE,

                ColorType::Reset => {
                    // init the original color in case it is not set.
                    let mut original_color = original_console_color();

                    const REMOVE_FG_MASK: u16 = FG_INTENSITY | FG_RED | FG_GREEN | FG_BLUE;
                    // remove all foreground values from the original color, we don't want to reset those.
                    original_color &= !(REMOVE_FG_MASK);
                    original_color
                }
                /* WinApi will be used for systems that do not support ANSI, those are windows version less then 10. RGB and 255 (AnsiBValue) colors are not supported in that case.*/
                ColorType::Rgb { r: _, g: _, b: _ } => 0,
                ColorType::AnsiValue(_val) => 0,
            };
        }
    };

    winapi_color
}

fn init_console_color() -> Result<()> {
    let screen_buffer = ScreenBuffer::current()?;

    let attr = screen_buffer.info()?.attributes();

    GET_ORIGINAL_CONSOLE_COLOR.call_once(|| {
        unsafe { ORIGINAL_CONSOLE_COLOR = attr };
    });
    Ok(())
}

fn original_console_color() -> u16 {
    return unsafe { ORIGINAL_CONSOLE_COLOR };
}

static GET_ORIGINAL_CONSOLE_COLOR: Once = Once::new();
static mut ORIGINAL_CONSOLE_COLOR: u16 = 0;

#[cfg(test)]
mod tests {
    use crate::color::winapi::{color_value, BG_INTENSITY, BG_RED, FG_INTENSITY, FG_RED};
    use crate::{ColorType, Colored};

    #[test]
    fn test_parse_fg_color() {
        let colored = Colored::Fg(ColorType::Red);
        assert_eq!(color_value(colored), FG_INTENSITY | FG_RED);
    }

    #[test]
    fn test_parse_bg_color() {
        let colored = Colored::Bg(ColorType::Red);
        assert_eq!(color_value(colored), BG_INTENSITY | BG_RED);
    }
}
