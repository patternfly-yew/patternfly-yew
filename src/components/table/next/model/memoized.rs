use crate::next::TableEntryRenderer;
use std::iter::Enumerate;
use std::rc::Rc;

pub struct SplitTableModel {}

pub struct MemoizedTableModel<T> {
    entries: Rc<Vec<T>>,
}

impl<T> Clone for MemoizedTableModel<T> {
    fn clone(&self) -> Self {
        Self {
            entries: self.entries.clone(),
        }
    }
}

impl<T> MemoizedTableModel<T> {
    pub fn new(entries: Rc<Vec<T>>) -> Self {
        Self { entries }
    }
}

impl<T> From<Rc<Vec<T>>> for MemoizedTableModel<T> {
    fn from(value: Rc<Vec<T>>) -> Self {
        MemoizedTableModel::new(value)
    }
}

impl<T> PartialEq for MemoizedTableModel<T> {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.entries, &other.entries)
    }
}

impl<C, T> super::TableDataModel<C> for MemoizedTableModel<T>
where
    C: Clone + Eq + 'static,
    T: TableEntryRenderer<C> + 'static,
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
