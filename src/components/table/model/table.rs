use super::{TableDataModel, TableModelEntry};
use crate::prelude::ExpansionState;
use std::cell::RefCell;
use std::collections::HashMap;
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
    state: Rc<RefCell<HashMap<M::Key, ExpansionState<C>>>>,
}

impl<C, M> StateModel<C, M>
where
    C: Clone + Eq + 'static,
    M: TableDataModel<C>,
{
    pub fn new(model: M, state: Rc<RefCell<HashMap<M::Key, ExpansionState<C>>>>) -> Self {
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
    type Iterator<'i> = StateModelIter<'i, Self::Key, Self::Item, C>;
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
            let expansion = state.get(&key).cloned();
            TableModelEntry {
                key,
                value,
                expansion,
            }
        }))
    }
}

pub struct StateModelIter<'i, K, V, C>(Box<dyn Iterator<Item = TableModelEntry<'i, V, K, C>> + 'i>)
where
    K: Into<Key>,
    C: Clone + Eq;

impl<'i, K, V, C> StateModelIter<'i, K, V, C>
where
    K: Into<Key>,
    C: Clone + Eq,
{
    pub fn new<I>(iter: I) -> Self
    where
        I: Iterator<Item = TableModelEntry<'i, V, K, C>> + 'i,
    {
        Self(Box::new(iter))
    }
}

impl<'i, K, V, C> Iterator for StateModelIter<'i, K, V, C>
where
    K: Into<Key>,
    C: Clone + Eq,
{
    type Item = TableModelEntry<'i, V, K, C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}
