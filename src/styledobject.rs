//! This module contains the logic to style an object that contains some 'content' which can be styled.

use std::fmt::{self, Display, Formatter};
use std::result;

use crossterm_utils::{csi, queue};

use super::{color, Attribute, ColorType, Colorize, ObjectStyle, SetBg, SetFg, Styler};

/// Contains both the style and the content which can be styled.
#[derive(Clone)]
pub struct StyledObject<D: Display + Clone> {
    pub object_style: ObjectStyle,
    pub content: D,
}

impl<'a, D: Display + 'a + Clone> StyledObject<D> {
    /// Set the foreground color with the given color
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// You can do: `with().on().attr()`
    pub fn with(mut self, foreground_color: ColorType) -> StyledObject<D> {
        self.object_style = self.object_style.set_fg(foreground_color);
        self
    }

    /// Set the background color with the given color
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// You can do: `with().on().attr()`
    pub fn on(mut self, background_color: ColorType) -> StyledObject<D> {
        self.object_style = self.object_style.set_bg(background_color);
        self
    }

    /// Add an attribute to the styled object.
    ///
    /// # Remarks
    ///
    /// This methods consumes 'self', and works like a builder.
    /// You can do: `with().on().attr()`
    pub fn attr(mut self, attr: Attribute) -> StyledObject<D> {
        self.object_style.add_attr(attr);
        self
    }
}

impl<D: Display + Clone> Display for StyledObject<D> {
    fn fmt(&self, f: &mut Formatter) -> result::Result<(), fmt::Error> {
        let colored_terminal = color();
        let mut reset = false;

        if let Some(bg) = self.object_style.bg_color {
            queue!(f, SetBg(bg)).map_err(|_| fmt::Error)?;
            reset = true;
        }
        if let Some(fg) = self.object_style.fg_color {
            queue!(f, SetFg(fg)).map_err(|_| fmt::Error)?;
            reset = true;
        }

        for attr in self.object_style.attrs.iter() {
            fmt::Display::fmt(&format!(csi!("{}m"), *attr as i16), f)?;
            reset = true;
        }

        fmt::Display::fmt(&self.content, f)?;

        if reset {
            colored_terminal.reset().map_err(|_| fmt::Error)?;
        }

        Ok(())
    }
}

impl<D: Display + Clone> Colorize<D> for StyledObject<D> {
    // foreground colors
    def_color!(fg_color: black => ColorType::Black);
    def_color!(fg_color: dark_grey => ColorType::DarkGrey);
    def_color!(fg_color: red => ColorType::Red);
    def_color!(fg_color: dark_red => ColorType::DarkRed);
    def_color!(fg_color: green => ColorType::Green);
    def_color!(fg_color: dark_green => ColorType::DarkGreen);
    def_color!(fg_color: yellow => ColorType::Yellow);
    def_color!(fg_color: dark_yellow => ColorType::DarkYellow);
    def_color!(fg_color: blue => ColorType::Blue);
    def_color!(fg_color: dark_blue => ColorType::DarkBlue);
    def_color!(fg_color: magenta => ColorType::Magenta);
    def_color!(fg_color: dark_magenta => ColorType::DarkMagenta);
    def_color!(fg_color: cyan => ColorType::Cyan);
    def_color!(fg_color: dark_cyan => ColorType::DarkCyan);
    def_color!(fg_color: white => ColorType::White);
    def_color!(fg_color: grey => ColorType::Grey);

    // background colors
    def_color!(bg_color: on_black => ColorType::Black);
    def_color!(bg_color: on_dark_grey => ColorType::DarkGrey);
    def_color!(bg_color: on_red => ColorType::Red);
    def_color!(bg_color: on_dark_red => ColorType::DarkRed);
    def_color!(bg_color: on_green => ColorType::Green);
    def_color!(bg_color: on_dark_green => ColorType::DarkGreen);
    def_color!(bg_color: on_yellow => ColorType::Yellow);
    def_color!(bg_color: on_dark_yellow => ColorType::DarkYellow);
    def_color!(bg_color: on_blue => ColorType::Blue);
    def_color!(bg_color: on_dark_blue => ColorType::DarkBlue);
    def_color!(bg_color: on_magenta => ColorType::Magenta);
    def_color!(bg_color: on_dark_magenta => ColorType::DarkMagenta);
    def_color!(bg_color: on_cyan => ColorType::Cyan);
    def_color!(bg_color: on_dark_cyan => ColorType::DarkCyan);
    def_color!(bg_color: on_white => ColorType::White);
    def_color!(bg_color: on_grey => ColorType::Grey);
}

impl<D: Display + Clone> Styler<D> for StyledObject<D> {
    def_attr!(reset => Attribute::Reset);
    def_attr!(bold => Attribute::Bold);
    def_attr!(underlined => Attribute::Underlined);
    def_attr!(reverse => Attribute::Reverse);
    def_attr!(dim => Attribute::Dim);
    def_attr!(italic => Attribute::Italic);
    def_attr!(negative => Attribute::Reverse);
    def_attr!(slow_blink => Attribute::SlowBlink);
    def_attr!(rapid_blink => Attribute::RapidBlink);
    def_attr!(hidden => Attribute::Hidden);
    def_attr!(crossed_out => Attribute::CrossedOut);
}

#[cfg(test)]
mod tests {
    use crate::{Attribute, ColorType, ObjectStyle};

    #[test]
    fn test_set_fg_bg_add_attr() {
        let mut object_style = ObjectStyle::new()
            .set_fg(ColorType::Blue)
            .set_bg(ColorType::Red);
        object_style.add_attr(Attribute::Reset);

        let mut styled_object = object_style.apply_to("test");

        styled_object = styled_object
            .with(ColorType::Green)
            .on(ColorType::Magenta)
            .attr(Attribute::NoItalic);

        assert_eq!(styled_object.object_style.fg_color, Some(ColorType::Green));
        assert_eq!(
            styled_object.object_style.bg_color,
            Some(ColorType::Magenta)
        );
        assert_eq!(styled_object.object_style.attrs.len(), 2);
        assert_eq!(styled_object.object_style.attrs[0], Attribute::Reset);
        assert_eq!(styled_object.object_style.attrs[1], Attribute::NoItalic);
    }
}
