use crate::prelude::{Cell, CellContext};
use std::rc::Rc;

/// A node in a tree
pub trait TreeNode<C> {
    fn render_cell(&self, ctx: CellContext<C>) -> Cell;

    fn children(&self) -> Vec<Rc<dyn TreeNode<C>>>;
}

/// A model providing access to tree nodes
pub trait TreeTableModel<C> {
    fn children(&self) -> Vec<Rc<dyn TreeNode<C>>>;
}
