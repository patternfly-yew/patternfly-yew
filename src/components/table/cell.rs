use super::props::TextModifier;
use yew::prelude::*;

/// A rendered cell.
#[derive(Debug, Default)]
pub struct Cell {
    pub content: Html,
    pub center: bool,
    pub text_modifier: Option<TextModifier>,
}

impl Cell {
    pub fn new(content: Html) -> Self {
        Self {
            content,
            ..Default::default()
        }
    }

    pub fn center(mut self) -> Self {
        self.center = true;
        self
    }

    pub fn text_modifier(mut self, text_modifier: impl Into<Option<TextModifier>>) -> Self {
        self.text_modifier = text_modifier.into();
        self
    }
}

impl From<Html> for Cell {
    fn from(content: Html) -> Self {
        Cell::new(content)
    }
}

/// The context information for rendering a cell.
#[derive(Copy, Clone, Debug)]
pub struct CellContext<'c, C> {
    pub column: &'c C,
}
