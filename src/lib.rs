//! # Styling Module
//!
//! Crossterm provides options for you to style your text and terminal. Take for example coloring output and applying attributes.
//!
//! **Color support**
//! Windows systems with versions less than 10 will only have support for 16 colors and don't have any support for attributes. Most UNIX-terminal is supporting lots of colors and attributes.
//!
//! ## Colors
//! There are 16 base colors which available for almost all terminals even windows 7 and 8.
//!
//! | Light Variant  | Dark Variant    |
//! | :-------------| :-------------   |
//! |       Grey     |      Black      |
//! |       Red      |      DarkRed    |
//! |       Green    |      DarkGreen  |
//! |       Yellow   |      DarkYellow |
//! |       Blue     |      DarkBlue   |
//! |       Magenta  |      DarkMagenta|
//! |       Cyan     |      DarkCyan   |
//! |       White    |      DarkWhite  |
//!
//! In addition to 16 colors, most UNIX terminals and Windows 10 consoles are also supporting more colors.
//! Those colors could be: [True color (24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit)) coloring scheme, which allows you to use [RGB](https://nl.wikipedia.org/wiki/RGB-kleursysteem), and [256 (Xterm, 8-bit)](https://jonasjacek.github.io/colors/) colors.
//! Checkout the [examples](https://github.com/crossterm-rs/crossterm/blob/master/examples/style.rs) on how to use this feature.
//!
//! ## Attributes
//! Only UNIX and Windows 10 terminals are supporting attributes on top of the text. Crossterm allows you to add attributes to the text.
//! Not all attributes are widely supported for all terminals, keep that in mind when working with this.
//!
//! Crossterm implements almost all attributes shown in this [Wikipedia-list](https://en.wikipedia.org/wiki/ANSI_escape_code#SGR_(Select_Graphic_Rendition)_parameters).
//!
//! | Attribute                      |     Support                                             |  Note         |
//! | :-------------:                |  :-------------:                                         | :-------------: |
//! |       Reset                    |  Windows, UNIX                                           |  This will reset all current set attributes.     |
//! |       Bold                     |  Windows, UNIX                                           |  This will increase the text sensitivity also known as bold.     |
//! |       Dim                      |  Windows, UNIX                                           |  This will decrease the text sensitivity also known as bold.   |
//! |       Italic                   |  Not widely supported, sometimes treated as inverse.     |  This will make the text italic.   |
//! |       Underlined               |  Windows, UNIX                                           |  A line under a word, especially in order to show its importance.   |
//! |       SlowBlink                |  Not widely supported, sometimes treated as inverse.     |  less than 150 per minute  |
//! |       RapidBlink               |  Not widely supported                                    |  MS-DOS ANSI.SYS; 150+ per minute;  |
//! |       Reverse                  |  Windows, UNIX                                           |   foreground and background colors |
//! |       Hidden                   |  Windows, UNIX |                                         |  Also known as 'Conceal'
//! |       Fraktur                  |  UNIX                                                    |  characters legible, but marked for deletion. |
//! |       DefaultForegroundColor   |  Unknown                                                 |  Implementation defined (according to standard) |
//! |       DefaultBackgroundColor   |  Unknown                                                 |  Implementation defined (according to standard) |
//! |       Framed                   |  Not widely supported                                    |  Framed text.
//! |       Encircled                |  Unknown                                                 |  This will turn on the encircled attribute. |
//! |       OverLined                |  Unknown                                                 |  This will draw a line at the top of the text. |
//!
//! (There are a few attributes who disable one of the above attributes, I did not write those down to keep the list short).
//!
//! Now we have covered the basics of styling lets go over to some examples.
//!
//!
//! # Example
//!
//! _setup the basics_
//! ```no_run
//! use crossterm_style::{Colored, Color, Attribute, Styler, Colorize};
//!
//! fn main() {
//!     /* your code here */
//! }
//! ```
//!
//! There are a couple of ways to style the terminal output with crossterm. The most important part of the styling module is `StyledObject`.
//!
//! A `StyledObject` is just a wrapper crossterm uses to store the text and style together.
//! A `StyledObject` implements `Display` and thus you could use it inside `print!`, `println!` etc.
//!
//! Without further ado let's get straight into it.
//!
//! ## Coloring
//!
//! There are a few ways to do the coloring, the first one is by using the `Colored` enum.
//!
//! ### Using Enum
//! ```no_run
//! use crossterm_style::{Colored, Color};
//! println!("{} Red foreground color", Colored::Fg(Color::Red));
//! println!("{} Blue background color", Colored::Bg(Color::Blue));
//! ```
//! `Colored::Bg` will set the background color, and `Colored::Fg` will set the foreground color to the provided color.
//! The provided color is of type `Color` and has a bunch of enum values you could choose out.
//!
//! Because `Colored` implements `Display` you are able to use it inside any write statement.
//!
//! ### Using Methods
//! You can do the same as the above in a slightly different way. Instead of enabling it for all text you could also color the only piece of text.
//! (Make sure to include the `crossterm::Coloring` trait).
//!
//! ```no_run
//! use crossterm_style::Colorize;
//! let styled_text = "Red forground color on blue background.".red().on_blue();
//! println!("{}", styled_text);
//! ```
//!
//! As you see in the above example you could call coloring methods on a string. How is this possible you might ask..?
//! Well, the trait `Coloring`, who you need to include, is implemented for `&'static str`.
//! When calling a method on this string crossterm transforms it into a `StyledObject` who you could use in your write statements.
//!
//!
//! ### RGB
//! Most UNIX terminals and all Windows 10 consoles are supporting [True color(24-bit)](https://en.wikipedia.org/wiki/Color_depth#True_color_(24-bit)) coloring scheme.
//! You can set the color of the terminal by using `Color::RGB(r,g,b)`.
//!
//! ```no_run
//! // custom rgb value (Windows 10 and UNIX systems)
//! use crossterm_style::{Colored, Color};
//! println!("{}{} 'Light green' text on 'Black' background", Colored::Fg(Color::Rgb { r: 0, g: 255, b: 128 }), Colored::Bg(Color::Rgb {r: 0, g: 0, b: 0}));
//! ```
//! This will print some light green text on black background.
//!
//! ### Custom ANSI color value
//! When working on UNIX or Windows 10 you could also specify a custom ANSI value ranging up from 0 to 256.
//! See [256 (Xterm, 8-bit) colors](https://jonasjacek.github.io/colors/) for more information.
//!
//! ```
//! // custom ansi color value (Windows 10 and UNIX systems)
//! use crossterm_style::{Colored, Color};
//! println!("{} some colored text", Colored::Fg(Color::AnsiValue(10)));
//! ```
//!
//! ## Attributes
//! When working with UNIX or Windows 10 terminals you could also use attributes to style your text. For example, you could cross your text with a line and make it bold.
//! See [this](styling.md#Attributes) for more information.
//!
//! ### Using Enum
//! You could use the `Attribute` enum for styling text with attributes.
//! `Attribute` implements `Display`, thus crossterm will enable the attribute style when using it in any writing operation.
//!
//! ```rust
//! use crossterm_style::Attribute;
//! println!(
//!     "{} Underlined {} No Underline",
//!     Attribute::Underlined,
//!     Attribute::NoUnderline
//! );
//! ```
//!
//! ### Using Method
//!
//! You can do the same as the above in a slightly different way. Instead of enabling it for all text you could also style only one piece of text.
//! (Make sure to include the `crossterm::Styler` trait).
//!
//! ```no_run
//! use crossterm_style::Styler;
//! println!("{}", "Bold text".bold());
//! println!("{}", "Underlined text".underlined());
//! println!("{}", "Negative text".negative());
//! ```
//!
//! ### Using Command API
//!
//! ```no_run
//! use std::io::{stdout, Write};
//!
//! use crossterm_utils::{execute, Result, Output};
//! use crossterm_style::{SetBg, SetFg, SetAttr, Color, Attribute};
//!
//! fn main() -> Result<()> {
//!     execute!(
//!         stdout(),
//!         SetFg(Color::Blue),
//!         SetBg(Color::Red),
//!         Output("Blue text on red background".to_string()),
//!         SetAttr(Attribute::Reset)
//!     )
//! }
//! ```
//!
//! As you see in the above example you could call attributes methods on a string. How is this possible you might ask..?
//! Well, the trait `Styling`, who you need to include, is implemented for `&'static str`.
//! When calling a method on any string crossterm transforms will transform it into a `StyledObject` who you could use in your write statements.
//!
//! ---------------------------------------------------------------------------------------------------------------------------------------------
//! More examples could be found at this [link](https://github.com/crossterm-rs/crossterm/blob/master/examples/style.rs).

#![deny(unused_imports)]

use std::env;
use std::fmt::Display;

#[cfg(windows)]
use crossterm_utils::supports_ansi;
pub use crossterm_utils::{
    execute, impl_display, queue, Command, ExecutableCommand, QueueableCommand, Result,
};

use style::ansi::{self, AnsiColor};
#[cfg(windows)]
use style::winapi::WinApiColor;
use style::Style;

pub use self::enums::{Attribute, Color, Colored};
pub use self::objectstyle::ObjectStyle;
pub use self::styledobject::StyledObject;
pub use self::traits::{Colorize, Styler};

#[macro_use]
mod macros;
mod enums;
mod objectstyle;
mod style;
mod styledobject;
mod traits;

/// This could be used to style a type that implements `Display` with colors and attributes.
///
/// # Example
/// ```no_run
/// // get a styled object which could be painted to the terminal.
/// use crossterm_style::{style, Color};
///
/// let styled_object = style("Some Blue colored text on black background")
///     .with(Color::Blue)
///     .on(Color::Black);
///
/// // print the styled text 10 * times to the current screen.
/// for i in 1..10
/// {
///     println!("{}", styled_object);
/// }
/// ```
pub fn style<'a, D: 'a>(val: D) -> StyledObject<D>
where
    D: Display + Clone,
{
    ObjectStyle::new().apply_to(val)
}

impl Colorize<&'static str> for &'static str {
    // foreground colors
    def_str_color!(fg_color: black => Color::Black);
    def_str_color!(fg_color: dark_grey => Color::DarkGrey);
    def_str_color!(fg_color: red => Color::Red);
    def_str_color!(fg_color: dark_red => Color::DarkRed);
    def_str_color!(fg_color: green => Color::Green);
    def_str_color!(fg_color: dark_green => Color::DarkGreen);
    def_str_color!(fg_color: yellow => Color::Yellow);
    def_str_color!(fg_color: dark_yellow => Color::DarkYellow);
    def_str_color!(fg_color: blue => Color::Blue);
    def_str_color!(fg_color: dark_blue => Color::DarkBlue);
    def_str_color!(fg_color: magenta => Color::Magenta);
    def_str_color!(fg_color: dark_magenta => Color::DarkMagenta);
    def_str_color!(fg_color: cyan => Color::Cyan);
    def_str_color!(fg_color: dark_cyan => Color::DarkCyan);
    def_str_color!(fg_color: white => Color::White);
    def_str_color!(fg_color: grey => Color::Grey);

    // background colors
    def_str_color!(bg_color: on_black => Color::Black);
    def_str_color!(bg_color: on_dark_grey => Color::DarkGrey);
    def_str_color!(bg_color: on_red => Color::Red);
    def_str_color!(bg_color: on_dark_red => Color::DarkRed);
    def_str_color!(bg_color: on_green => Color::Green);
    def_str_color!(bg_color: on_dark_green => Color::DarkGreen);
    def_str_color!(bg_color: on_yellow => Color::Yellow);
    def_str_color!(bg_color: on_dark_yellow => Color::DarkYellow);
    def_str_color!(bg_color: on_blue => Color::Blue);
    def_str_color!(bg_color: on_dark_blue => Color::DarkBlue);
    def_str_color!(bg_color: on_magenta => Color::Magenta);
    def_str_color!(bg_color: on_dark_magenta => Color::DarkMagenta);
    def_str_color!(bg_color: on_cyan => Color::Cyan);
    def_str_color!(bg_color: on_dark_cyan => Color::DarkCyan);
    def_str_color!(bg_color: on_white => Color::White);
    def_str_color!(bg_color: on_grey => Color::Grey);
}

impl Styler<&'static str> for &'static str {
    def_str_attr!(reset => Attribute::Reset);
    def_str_attr!(bold => Attribute::Bold);
    def_str_attr!(underlined => Attribute::Underlined);
    def_str_attr!(reverse => Attribute::Reverse);
    def_str_attr!(dim => Attribute::Dim);
    def_str_attr!(italic => Attribute::Italic);
    def_str_attr!(negative => Attribute::Reverse);
    def_str_attr!(slow_blink => Attribute::SlowBlink);
    def_str_attr!(rapid_blink => Attribute::RapidBlink);
    def_str_attr!(hidden => Attribute::Hidden);
    def_str_attr!(crossed_out => Attribute::CrossedOut);
}

/// Allows you to style the terminal.
///
/// # Features:
///
/// - Foreground color (16 base colors)
/// - Background color (16 base colors)
/// - 256 color support (Windows 10 and UNIX only)
/// - RGB support (Windows 10 and UNIX only)
/// - Text Attributes like: bold, italic, underscore and crossed word ect (Windows 10 and UNIX only)
///
/// Check [examples](https://github.com/crossterm-rs/examples) in the library for more specific examples.
///
/// ## Examples
///
/// Basic usage:
///
/// ```no_run
/// // You can replace the following line with `use crossterm::TerminalColor;`
/// // if you're using the `crossterm` crate with the `style` feature enabled.
/// use crossterm_style::{Result, TerminalColor, Color};
///
/// fn main() -> Result<()> {
///     let color = TerminalColor::new();
///     // set foreground color
///     color.set_fg(Color::Blue)?;
///     // set background color
///     color.set_bg(Color::Red)?;
///     // reset to the default colors
///     color.reset()
/// }
/// ```
pub struct TerminalColor {
    #[cfg(windows)]
    color: Box<(dyn Style + Sync + Send)>,
    #[cfg(unix)]
    color: AnsiColor,
}

impl TerminalColor {
    /// Creates a new `TerminalColor`
    pub fn new() -> TerminalColor {
        #[cfg(windows)]
        let color = if supports_ansi() {
            Box::from(AnsiColor::new()) as Box<(dyn Style + Sync + Send)>
        } else {
            WinApiColor::new() as Box<(dyn Style + Sync + Send)>
        };

        #[cfg(unix)]
        let color = AnsiColor::new();

        TerminalColor { color }
    }

    /// Set the foreground color to the given color.
    pub fn set_fg(&self, color: Color) -> Result<()> {
        self.color.set_fg(color)
    }

    /// Set the background color to the given color.
    pub fn set_bg(&self, color: Color) -> Result<()> {
        self.color.set_bg(color)
    }

    /// Reset the terminal colors and attributes to default.
    pub fn reset(&self) -> Result<()> {
        self.color.reset()
    }

    /// Get available color count.
    ///
    /// # Remarks
    ///
    /// This does not always provide a good result.
    pub fn available_color_count(&self) -> u16 {
        env::var("TERM")
            .map(|x| if x.contains("256color") { 256 } else { 8 })
            .unwrap_or(8)
    }
}

/// Get a `TerminalColor` implementation whereon color related actions can be performed.
pub fn color() -> TerminalColor {
    TerminalColor::new()
}

/// When executed, this command will set the foreground color of the terminal to the given color.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetFg(pub Color);

impl Command for SetFg {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_fg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_fg(self.0)
    }
}

/// When executed, this command will set the background color of the terminal to the given color.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetBg(pub Color);

impl Command for SetBg {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_bg_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().set_bg(self.0)
    }
}

/// When executed, this command will set the given attribute to the terminal.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct SetAttr(pub Attribute);

impl Command for SetAttr {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::set_attr_csi_sequence(self.0)
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        // attributes are not supported by WinAPI.
        Ok(())
    }
}

/// When executed, this command will print the styled font to the terminal.
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct PrintStyledFont<D: Display + Clone>(pub StyledObject<D>);

impl<D> Command for PrintStyledFont<D>
where
    D: Display + Clone,
{
    type AnsiType = StyledObject<D>;

    fn ansi_code(&self) -> Self::AnsiType {
        self.0.clone()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        Ok(())
    }
}

/// When executed, this command will reset the console colors back to default
///
/// See `crossterm/examples/command.rs` for more information on how to execute commands.
pub struct ResetColor;

impl Command for ResetColor {
    type AnsiType = String;

    fn ansi_code(&self) -> Self::AnsiType {
        ansi::RESET_CSI_SEQUENCE.to_string()
    }

    #[cfg(windows)]
    fn execute_winapi(&self) -> Result<()> {
        WinApiColor::new().reset()
    }
}

impl_display!(for SetFg);
impl_display!(for SetBg);
impl_display!(for SetAttr);
impl_display!(for PrintStyledFont<String>);
impl_display!(for PrintStyledFont<&'static str>);
impl_display!(for ResetColor);
