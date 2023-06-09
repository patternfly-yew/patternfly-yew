use super::{StateModel, TableDataModel};
use crate::{StateModelIter, TableModel};
use std::collections::HashSet;
use std::hash::Hash;
use std::rc::Rc;
use yew::prelude::*;

#[hook]
pub fn use_table_data<C, M>(data: M) -> (UseTableData<C, M>, Callback<(M::Key, bool)>)
where
    C: Clone + Eq + 'static,
    M: PartialEq + Clone + TableDataModel<C> + 'static,
    M::Key: Hash,
{
    let state = use_mut_ref(HashSet::<M::Key>::new);
    let model = {
        let state = state.clone();
        use_memo(
            move |model| {
                state.borrow_mut().retain(|key| model.contains(key));
                StateModel::new(model.clone(), state)
            },
            data,
        )
    };

    let trigger = use_force_update();

    let ontoggle = Callback::from(move |(key, expanded): (M::Key, bool)| {
        let changed = match expanded {
            true => state.borrow_mut().insert(key),
            false => state.borrow_mut().remove(&key),
        };
        if changed {
            trigger.force_update();
        }
    });

    ({ UseTableData { model } }, ontoggle)
}

pub struct UseTableData<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + Clone + TableDataModel<C> + 'static,
    M::Key: Hash,
{
    model: Rc<StateModel<C, M>>,
}

impl<C, M> TableModel<C> for UseTableData<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + Clone + TableDataModel<C> + 'static,
    M::Key: Hash,
{
    type Iterator<'i> = StateModelIter<'i, M::Key, M::Item>;
    type Item = M::Item;
    type Key = M::Key;

    fn len(&self) -> usize {
        self.model.len()
    }

    fn is_empty(&self) -> bool {
        self.model.is_empty()
    }

    fn iter(&self) -> Self::Iterator<'_> {
        self.model.iter()
    }
}

impl<C, M> Clone for UseTableData<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + Clone + TableDataModel<C> + 'static,
    M::Key: Hash,
{
    fn clone(&self) -> Self {
        Self {
            model: self.model.clone(),
        }
    }
}

impl<C, M> PartialEq for UseTableData<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + Clone + TableDataModel<C> + 'static,
    M::Key: Hash,
{
    fn eq(&self, other: &Self) -> bool {
        self.model == other.model
    }
}
