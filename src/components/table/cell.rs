use yew::prelude::*;

/// A rendered cell.
#[derive(Debug, Default)]
pub struct Cell {
    pub content: Html,
    pub center: bool,
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
}

impl From<Html> for Cell {
    fn from(content: Html) -> Self {
        Cell::new(content)
    }
}

/// The context information for rendering a cell.
#[derive(Copy, Clone, Debug)]
pub struct CellContext {
    pub column: usize,
}
