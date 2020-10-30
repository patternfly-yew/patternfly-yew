use std::fmt::Debug;
use std::ops::Deref;
use std::sync::{Arc, RwLock};
use yew::prelude::*;

use super::TableRenderer;

pub trait TableModel: Debug + Default + PartialEq + Clone {
    type Item: TableRenderer;

    /// Get the number of items
    fn len(&self) -> usize;
    /// Test if the entry is expanded
    fn is_expanded(&self, index: usize) -> bool;
    /// Set the expanded state of the entry
    fn set_expanded(&mut self, index: usize, state: bool) -> ShouldRender;
    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<Self::Item>) -> R;
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableModelEntry<T> {
    pub value: T,
    pub expanded: bool,
    pub(crate) index: usize,
}

impl<T> TableModelEntry<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            expanded: false,
            index: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    entries: Vec<TableModelEntry<T>>,
}

impl<T> Default for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

impl<T> From<Vec<T>> for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    fn from(entries: Vec<T>) -> Self {
        let mut result = Vec::with_capacity(entries.len());

        let mut index = 0;
        for e in entries {
            result.push(TableModelEntry {
                value: e,
                index,
                expanded: false,
            });
            index += 1;
        }

        Self { entries: result }
    }
}

impl<T> TableModel for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.entries.is_expanded(index)
    }

    fn set_expanded(&mut self, index: usize, state: bool) -> bool {
        self.entries.set_expanded(index, state)
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<T>) -> R,
    {
        self.entries.map(f)
    }
}

impl<T> TableModel for Vec<TableModelEntry<T>>
where
    T: TableRenderer + Clone + Debug + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.get(index).map(|e| e.expanded).unwrap_or(false)
    }

    fn set_expanded(&mut self, index: usize, state: bool) -> bool {
        if let Some(entry) = self.get_mut(index) {
            if entry.expanded != state {
                entry.expanded = state;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<T>) -> R,
    {
        let mut result = Vec::new();
        for entry in self {
            result.push(f(entry));
        }
        result
    }
}

#[derive(Clone, Debug)]
pub struct SharedTableModel<T> {
    entries: Arc<RwLock<Vec<TableModelEntry<T>>>>,
}

impl<T> From<Vec<T>> for SharedTableModel<T> {
    fn from(initial_entries: Vec<T>) -> Self {
        let mut entries = Vec::with_capacity(initial_entries.len());
        let mut index = 0;

        for entry in initial_entries {
            entries.push(Self::new_entry(entry, index));
            index += 1;
        }

        Self {
            entries: Arc::new(RwLock::new(entries)),
        }
    }
}

impl<T> SharedTableModel<T> {
    fn new_entry(entry: T, index: usize) -> TableModelEntry<T> {
        TableModelEntry {
            expanded: false,
            value: entry,
            index,
        }
    }

    pub fn push(&mut self, entry: T) {
        let mut entries = self.entries.write().unwrap();
        let index = entries.len();
        entries.push(Self::new_entry(entry, index))
    }

    pub fn insert(&mut self, index: usize, entry: T) {
        let mut entries = self.entries.write().unwrap();
        entries.insert(index, Self::new_entry(entry, index));

        // now we need to re-index everything after index

        for entry in &mut entries[index + 1..] {
            entry.index += 1;
        }
    }

    pub fn pop(&mut self) -> Option<TableModelEntry<T>> {
        let mut entries = self.entries.write().unwrap();
        entries.pop()
    }
}

impl<T> Default for SharedTableModel<T> {
    fn default() -> Self {
        SharedTableModel::from(Vec::new())
    }
}

impl<T> PartialEq for SharedTableModel<T>
where
    T: Clone + PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        let others: Vec<_> = {
            other
                .entries
                .read()
                .map(|other| other.clone())
                .unwrap_or(vec![])
        };
        self.entries
            .read()
            .map(|entries| entries.deref().eq(&others))
            .unwrap_or(false)
    }
}

impl<T> TableModel for SharedTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.entries.read().map(|e| e.len()).unwrap_or_default()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.entries
            .read()
            .map(|e| e.is_expanded(index))
            .unwrap_or(false)
    }

    fn set_expanded(&mut self, index: usize, state: bool) -> bool {
        let mut entries = self.entries.write().unwrap();

        if let Some(entry) = entries.get_mut(index) {
            if entry.expanded != state {
                entry.expanded = state;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<T>) -> R,
    {
        let mut result = Vec::new();
        self.entries
            .read()
            .map(|entries| {
                for entry in entries.iter() {
                    result.push(f(entry));
                }
            })
            .unwrap_or_default();
        result
    }
}
