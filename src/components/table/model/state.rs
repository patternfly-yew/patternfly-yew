use crate::prelude::{TableEntryRenderer, TableModelEntry};
use std::iter::Enumerate;
use yew::UseStateHandle;
use yew::virtual_dom::Key;

#[derive(Clone)]
pub struct UseStateTableModel<T>
where
    T: PartialEq,
{
    entries: UseStateHandle<Vec<T>>,
}

impl<T> UseStateTableModel<T>
where
    T: PartialEq,
{
    pub fn new(entries: UseStateHandle<Vec<T>>) -> Self {
        Self { entries }
    }
}

impl<T> PartialEq for UseStateTableModel<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.entries.eq(&other.entries)
    }
}

impl<T> From<UseStateHandle<Vec<T>>> for UseStateTableModel<T>
where
    T: PartialEq,
{
    fn from(value: UseStateHandle<Vec<T>>) -> Self {
        UseStateTableModel::new(value)
    }
}

impl<C, T> super::TableDataModel<C> for UseStateTableModel<T>
where
    C: Clone + Eq + 'static,
    T: PartialEq + TableEntryRenderer<C> + Clone + 'static,
{
    type Iterator<'i> = Enumerate<core::slice::Iter<'i, Self::Item>>;
    type Item = T;
    type Key = usize;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_empty(&self) -> bool {
        self.entries.is_empty()
    }

    fn contains(&self, key: &Self::Key) -> bool {
        *key < self.entries.len()
    }

    fn iter(&self) -> Self::Iterator<'_> {
        self.entries.iter().enumerate()
    }
}

pub struct StateIter<'i, T, K, C>
where
    K: Into<Key>,
    C: Clone + Eq,
{
    iter: Box<dyn Iterator<Item = TableModelEntry<'i, T, K, C>> + 'i>,
}

impl<'i, T, K, C> Iterator for StateIter<'i, T, K, C>
where
    K: Into<Key>,
    C: Clone + Eq,
{
    type Item = TableModelEntry<'i, T, K, C>;

    fn next(&mut self) -> Option<Self::Item> {
        self.iter.next()
    }
}
