use crate::prelude::{Cell, CellContext};
use std::rc::Rc;

/// A node in a tree
pub trait TreeNode {
    fn render_main(&self) -> Cell;

    fn render_cell(&self, ctx: CellContext) -> Cell;

    fn children(&self) -> Vec<Rc<dyn TreeNode>>;
}

/// A model providing access to tree nodes
pub trait TreeTableModel {
    fn children(&self) -> Vec<Rc<dyn TreeNode>>;
}
