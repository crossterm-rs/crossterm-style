# Version master

- Added unit tests 
- Restructured files
- Removed unsafe static code
- Improved documentation and added book page to lib.rs

# Version 0.5.1

- Maintenance release only
- Moved to a [separate repository](https://github.com/crossterm-rs/crossterm-style)

# Version 0.5.0

- `get_available_color_count` returns no result
- `ExecutableCommand::queue` returns `crossterm::Result`
- `QueueableCommand::queue` returns `crossterm::Result`
- `available_color_count` to `available_color_count()`
- Added derives: `Debug` for `ObjectStyle`  [debug-derive]
- Command API takes mutable self instead of self

# Version 0.3.0

- Removed `TerminalColor::from_output()` 
- Added `NoItalic` attribute

# Version 0.2.0

- Introduced more `Attributes`
- Introduced easier ways to style text [issue 87](https://github.com/crossterm-rs/crossterm/issues/87).
- Removed `ColorType` since it was unnecessary.

# Version 0.1.0

- Moved out of `crossterm` 5.4 crate. 
