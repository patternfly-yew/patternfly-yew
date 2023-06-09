use super::{TableDataModel, TableModelEntry};
use std::cell::RefCell;
use std::collections::HashSet;
use std::hash::Hash;
use std::marker::PhantomData;
use std::rc::Rc;
use yew::virtual_dom::Key;

/// A [`super::TableModel`] based on a [`TableDataModel`] plus additional state.
pub struct StateModel<C, M>
where
    C: Clone + Eq + 'static,
    M: TableDataModel<C>,
{
    _marker: PhantomData<C>,
    model: M,
    state: Rc<RefCell<HashSet<M::Key>>>,
}

impl<C, M> StateModel<C, M>
where
    C: Clone + Eq + 'static,
    M: TableDataModel<C>,
{
    pub fn new(model: M, state: Rc<RefCell<HashSet<M::Key>>>) -> Self {
        Self {
            model,
            state,
            _marker: Default::default(),
        }
    }
}

impl<C, M> PartialEq for StateModel<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableDataModel<C>,
    M::Key: Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.state == other.state && self.model == other.model
    }
}

impl<C, M> super::TableModel<C> for StateModel<C, M>
where
    C: Clone + Eq + 'static,
    M: TableDataModel<C> + 'static,
    M::Key: Hash,
{
    type Iterator<'i> = StateModelIter<'i, Self::Key, Self::Item>;
    type Item = M::Item;
    type Key = M::Key;

    fn len(&self) -> usize {
        self.model.len()
    }

    fn is_empty(&self) -> bool {
        self.model.is_empty()
    }

    fn iter(&self) -> Self::Iterator<'_> {
        let state = self.state.borrow().clone();
        StateModelIter::new(self.model.iter().map(move |(key, value)| {
            let expanded = state.contains(&key);
            TableModelEntry {
                key,
                value,
                expanded,
            }
        }))
    }
}

pub struct StateModelIter<'i, K, V>(Box<dyn Iterator<Item = TableModelEntry<'i, V, K>> + 'i>)
where
    K: Into<Key>;

impl<'i, K, V> StateModelIter<'i, K, V>
where
    K: Into<Key>,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = TableModelEntry<'i, V, K>> + 'i,
    {
        Self(Box::new(iter))
    }
}

impl<'i, K, V> Iterator for StateModelIter<'i, K, V>
where
    K: Into<Key>,
{
    type Item = TableModelEntry<'i, V, K>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
