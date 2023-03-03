use super::TableEntryRenderer;
use std::{
    fmt::{Debug, Formatter},
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, RwLock,
    },
};

/// A model providing data for a table.
pub trait TableModel: Default + PartialEq {
    type Item: TableEntryRenderer;

    /// Get the number of items
    fn len(&self) -> usize;

    /// Test if the table model is empty
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Test if the entry is expanded
    fn is_expanded(&self, index: usize) -> bool;

    /// Set the expanded state of the entry
    fn set_expanded(&self, index: usize, state: bool) -> bool;

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<Self::Item>) -> R;
}

#[derive(Clone, PartialEq)]
pub struct TableModelEntry<T> {
    pub value: T,
    pub expanded: bool,
    pub(crate) index: usize,
}

impl<T> Debug for TableModelEntry<T> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("TableModelEntry")
            .field("value", &"[..]")
            .field("expanded", &self.expanded)
            .field("index", &self.index)
            .finish()
    }
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

#[derive(Clone)]
pub struct SharedTableModel<T> {
    entries: Arc<RwLock<Vec<TableModelEntry<T>>>>,
    generation: usize,
    id: usize,
}

static ID: AtomicUsize = AtomicUsize::new(0);

impl<T> From<Vec<T>> for SharedTableModel<T> {
    fn from(initial_entries: Vec<T>) -> Self {
        let mut entries = Vec::with_capacity(initial_entries.len());

        for (index, entry) in initial_entries.into_iter().enumerate() {
            entries.push(Self::new_entry(entry, index));
        }

        let id = ID.fetch_add(1, Ordering::SeqCst);

        Self {
            entries: Arc::new(RwLock::new(entries)),
            generation: 0,
            id,
        }
    }
}

impl<T> SharedTableModel<T> {
    pub fn new(entries: Vec<T>) -> Self {
        entries.into()
    }

    fn new_entry(entry: T, index: usize) -> TableModelEntry<T> {
        TableModelEntry {
            expanded: false,
            value: entry,
            index,
        }
    }

    pub fn push(&mut self, entry: T) {
        let mut entries = self.entries.write().unwrap();
        self.generation += 1;
        let index = entries.len();
        entries.push(Self::new_entry(entry, index))
    }

    pub fn insert(&mut self, index: usize, entry: T) {
        let mut entries = self.entries.write().unwrap();
        self.generation += 1;

        entries.insert(index, Self::new_entry(entry, index));

        // now we need to re-index everything after insert

        for entry in &mut entries[index + 1..] {
            entry.index += 1;
        }
    }

    pub fn pop(&mut self) -> Option<TableModelEntry<T>> {
        let mut entries = self.entries.write().unwrap();
        self.generation += 1;
        entries.pop()
    }

    pub fn clear(&mut self) {
        let entries = self.entries.write();
        self.generation += 1;
        entries.unwrap().clear();
    }
}

impl<T> Default for SharedTableModel<T> {
    fn default() -> Self {
        SharedTableModel::from(Vec::new())
    }
}

/// Shared models are equal by their instance id and generation. NOT by their content.
///
/// This is required, because components need to refresh when their view on the model has changed,
/// as the content only exists once in memory.
impl<T> PartialEq for SharedTableModel<T>
where
    T: PartialEq,
{
    fn eq(&self, other: &Self) -> bool {
        self.generation == other.generation && self.id == other.id
    }
}

impl<T> TableModel for SharedTableModel<T>
where
    T: TableEntryRenderer + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.entries.read().map(|e| e.len()).unwrap_or_default()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.entries
            .read()
            .ok()
            .and_then(|e| e.get(index).map(|e| e.expanded))
            .unwrap_or(false)
    }

    fn set_expanded(&self, index: usize, state: bool) -> bool {
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

#[cfg(test)]
mod test {
    use crate::SharedTableModel;

    #[test]
    fn test() {
        // create two linked models
        let mut m1 = SharedTableModel::<String>::default();
        let m2 = m1.clone();

        // push data
        m1.push("Foo".into());

        // the models must not be equal, as one of them was modified
        assert!(m1 != m2);

        // when we clone it again, they must be equal
        let m3 = m1.clone();
        assert!(m1 == m3);
    }

    #[test]
    fn test_different_models() {
        let m1 = SharedTableModel::<String>::default();
        let m2 = SharedTableModel::<String>::default();
        assert!(m1 != m2);
    }
}
