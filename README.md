![Lines of Code][s7] [![Latest Version][s1]][l1] [![MIT][s2]][l2] [![docs][s3]][l3] [![Join us on Discord][s5]][l5]

# Crossterm Style

This crate allows you to work with the terminal colors and text attributes. It supports all UNIX
and Windows terminals down to Windows 7 (not all terminals are tested, see the
[Tested Terminals](https://github.com/crossterm-rs/crossterm/blob/master/README.md#tested-terminals) for more info).

`crossterm_style` is a sub-crate of the [crossterm](https://crates.io/crates/crossterm) crate. You can use it
directly, but it's **highly recommended** to use the [crossterm](https://crates.io/crates/crossterm) crate with
the `style` feature enabled (see [feature flags](https://crossterm-rs.github.io/crossterm/docs/feature_flags.html)
for more info).

## Future

> The `crossterm_style` crate code will be moved to the `crossterm` crate (it's reexported there now).
> Date is not set yet, but it doesn't make a lot of sense to start a new project with it. Please, use
> the `crossterm` crate with the `style` feature enabled.

Issues in this repository are disabled for the same reason. Please, report all issues in the
[crossterm-rs/crossterm](https://github.com/crossterm-rs/crossterm/issues) repository.

## Features

- Cross-platform
- Multi-threaded (send, sync)
- Detailed documentation
- Few dependencies
- Styled output
  - Foreground color (16 base colors)
  - Background color (16 base colors)
  - 256 (ANSI) color support (Windows 10 and UNIX only)
  - RGB color support (Windows 10 and UNIX only)
  - Text attributes
    - Bold, italic, underscore and crossed word
    - [More attributes](https://crossterm-rs.github.io/crossterm/docs/styling.html#attributes) (Windows 10 and UNIX only)

## Getting Started

<details>
<summary>
Click to show Cargo.toml.
</summary>

```toml
[dependencies]
# All crossterm features are enabled by default.
crossterm = "0.11"
```

</details>
<p></p>

### Attributes

```rust
use crossterm::{Colored, Color, Colorize, Styler, Attribute};

// pass any `Attribute` value to the formatting braces.
println!("{} Underlined {} No Underline", Attribute::Underlined, Attribute::NoUnderline);

// you could also call different attribute methods on a `&str` and keep on chaining if needed.
let styled_text = "Bold Underlined".bold().underlined();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").bold().underlined();
```

### Colors

```rust
use crossterm::{Colored, Color, Colorize};

println!("{} Red foreground color", Colored::Fg(Color::Red));
println!("{} Blue background color", Colored::Bg(Color::Blue));

// you can also call different coloring methods on a `&str`.
let styled_text = "Bold Underlined".red().on_blue();
println!("{}", styled_text);

// old-way but still usable
let styled_text = style("Bold Underlined").with(Color::Red).on(Color::Blue);
```

### RGB Colors and ANSI Values

```rust
use crossterm::{Colored, Color, Colorize};

// custom rgb value (Windows 10 and UNIX systems)
println!("{} some colored text", Colored::Fg(Color::Rgb {
    r: 10,
    g: 10,
    b: 10
}));

// custom ansi color value (Windows 10 and UNIX systems)
println!("{} some colored text", Colored::Fg(Color::AnsiValue(10)));
```

## Other Resources

- [API documentation](https://docs.rs/crossterm_style/) (with other examples)
- [Examples repository](https://github.com/crossterm-rs/examples)
- [The Book](https://crossterm-rs.github.io/crossterm/docs/index.html)

## Authors

* **Timon Post** - *Project Owner & creator*

## License

This project is licensed under the MIT License - see the [LICENSE](./LICENSE) file for details

[s1]: https://img.shields.io/crates/v/crossterm_style.svg
[l1]: https://crates.io/crates/crossterm_style

[s2]: https://img.shields.io/badge/license-MIT-blue.svg
[l2]: ./LICENSE

[s3]: https://docs.rs/crossterm_style/badge.svg
[l3]: https://docs.rs/crossterm_style/

[s5]: https://img.shields.io/discord/560857607196377088.svg?logo=discord
[l5]: https://discord.gg/K4nyTDB

[s7]: https://travis-ci.org/crossterm-rs/crossterm.svg?branch=master
