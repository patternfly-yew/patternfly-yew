mod hook;
mod memoized;
mod state;
mod table;

pub use hook::*;
pub use memoized::*;
pub use state::*;
use std::fmt::Debug;
pub use table::*;

use super::TableEntryRenderer;
use std::rc::Rc;
use yew::virtual_dom::Key;

/// A model providing data for a table.
pub trait TableModel<C>
where
    C: Clone + Eq + 'static,
{
    type Iterator<'i>: Iterator<Item = TableModelEntry<'i, Self::Item, Self::Key, C>>
    where
        Self: 'i;
    type Item: TableEntryRenderer<C> + 'static;
    type Key: Into<Key> + Clone + Debug + Eq + 'static;

    /// Get the number of items
    fn len(&self) -> usize;

    /// Test if the table model is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Iterate over all the items
    fn iter(&self) -> Self::Iterator<'_>;
}

impl<C, M> TableModel<C> for Rc<M>
where
    C: Clone + Eq + 'static,
    M: TableModel<C> + 'static,
{
    type Iterator<'i> = M::Iterator<'i>;
    type Item = M::Item;
    type Key = M::Key;

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_ref().iter()
    }
}

pub trait TableDataModel<C>
where
    C: Clone + Eq + 'static,
{
    type Iterator<'i>: Iterator<Item = (Self::Key, &'i Self::Item)>
    where
        Self: 'i;
    type Item: TableEntryRenderer<C> + 'static;
    type Key: Into<Key> + Clone + Debug + Eq + 'static;

    /// Get the number of items
    fn len(&self) -> usize;

    /// Test if the model is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Test if the model contains the key
    fn contains(&self, key: &Self::Key) -> bool;

    /// Iterate over all the items
    fn iter(&self) -> Self::Iterator<'_>;
}

impl<C, M> TableDataModel<C> for Rc<M>
where
    C: Clone + Eq + 'static,
    M: TableDataModel<C> + 'static,
{
    type Iterator<'i> = M::Iterator<'i>;
    type Item = M::Item;
    type Key = M::Key;

    fn len(&self) -> usize {
        self.as_ref().len()
    }

    fn is_empty(&self) -> bool {
        self.as_ref().is_empty()
    }

    fn contains(&self, key: &Self::Key) -> bool {
        self.as_ref().contains(key)
    }

    fn iter(&self) -> Self::Iterator<'_> {
        self.as_ref().iter()
    }
}

pub struct TableModelEntry<'t, T, K, C>
where
    K: Into<Key>,
    C: Clone + Eq,
{
    pub value: &'t T,
    pub key: K,
    pub expansion: Option<ExpansionState<C>>,
}
