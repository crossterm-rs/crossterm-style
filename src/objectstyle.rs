//! This module contains the `object style` that can be applied to an `styled object`.

use std::fmt::Display;

use super::{Attribute, Color, StyledObject};

/// Struct that contains the style properties that can be applied to a displayable object.
#[derive(Debug, Clone, Default)]
pub struct ObjectStyle {
    pub fg_color: Option<Color>,
    pub bg_color: Option<Color>,
    pub attrs: Vec<Attribute>,
}

impl ObjectStyle {
    /// Apply a `StyledObject` to the passed displayable object.
    pub fn apply_to<D: Display + Clone>(&self, val: D) -> StyledObject<D> {
        StyledObject {
            object_style: self.clone(),
            content: val,
        }
    }

    /// Get a new instance of `ObjectStyle`
    pub fn new() -> ObjectStyle {
        ObjectStyle::default()
    }

    /// Set the background color of `ObjectStyle` to the passed color.
    pub fn bg(mut self, color: Color) -> ObjectStyle {
        self.bg_color = Some(color);
        self
    }

    /// Set the foreground color of `ObjectStyle` to the passed color.
    pub fn fg(mut self, color: Color) -> ObjectStyle {
        self.fg_color = Some(color);
        self
    }

    /// Add an `Attribute` to the current text. Like italic or bold.
    pub fn add_attr(&mut self, attr: Attribute) {
        self.attrs.push(attr);
    }
}
