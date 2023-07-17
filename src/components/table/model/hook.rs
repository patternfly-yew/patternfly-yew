use super::{StateModel, TableDataModel};
use crate::prelude::{StateModelIter, TableModel};
use std::collections::HashSet;
use std::fmt::{Debug, Formatter};
use std::hash::Hash;
use std::rc::Rc;
use yew::html::IntoPropValue;
use yew::prelude::*;

pub struct OnToggleCallback<C, M>(pub Callback<(M::Key, bool)>)
where
    C: Clone + Eq + 'static,
    M: TableModel<C>;

impl<C, M> Debug for OnToggleCallback<C, M>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_tuple("OnToggleCallback").field(&self.0).finish()
    }
}

impl<C, M> PartialEq for OnToggleCallback<C, M>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn eq(&self, other: &Self) -> bool {
        self.0.eq(&other.0)
    }
}

impl<C, M> Default for OnToggleCallback<C, M>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn default() -> Self {
        Self(Default::default())
    }
}

impl<C, M> Clone for OnToggleCallback<C, M>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn clone(&self) -> Self {
        Self(self.0.clone())
    }
}

impl<C, M> IntoPropValue<OnToggleCallback<C, M>> for Callback<(M::Key, bool)>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn into_prop_value(self) -> OnToggleCallback<C, M> {
        OnToggleCallback(self)
    }
}

impl<C, M> IntoPropValue<OnToggleCallback<C, M>> for Rc<Callback<(M::Key, bool)>>
where
    C: Clone + Eq + 'static,
    M: TableModel<C>,
{
    fn into_prop_value(self) -> OnToggleCallback<C, M> {
        OnToggleCallback((*self).clone())
    }
}

#[hook]
pub fn use_table_data<C, M>(data: M) -> (UseTableData<C, M>, Rc<Callback<(M::Key, bool)>>)
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

    // FIXME: allow toggling entries without re-evaluating the whole table: https://github.com/patternfly-yew/patternfly-yew/issues/69
    /*
    let ontoggle = use_memo(
        |()| {
            Callback::from(move |(key, expanded): (M::Key, bool)| {
                let changed = match expanded {
                    true => state.borrow_mut().insert(key),
                    false => state.borrow_mut().remove(&key),
                };
                if changed {
                    trigger.force_update();
                }
            })
        },
        (),
    );*/

    let ontoggle = Rc::new(Callback::from(move |(key, expanded): (M::Key, bool)| {
        let changed = match expanded {
            true => state.borrow_mut().insert(key),
            false => state.borrow_mut().remove(&key),
        };
        if changed {
            trigger.force_update();
        }
    }));

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

#[cfg(test)]
mod test {
    use super::*;
    use crate::prelude::{
        Cell, CellContext, OnToggleCallback, TableEntryRenderer, TableModelEntry,
    };

    #[test]
    fn test_eq() {
        #[derive(Clone)]
        struct MockModel {}

        impl TableEntryRenderer<()> for String {
            fn render_cell(&self, _context: CellContext<'_, ()>) -> Cell {
                html!().into()
            }
        }

        impl TableModel<()> for MockModel {
            type Iterator<'i>  = std::vec::IntoIter<TableModelEntry<'i, Self::Item, Self::Key>> where
            Self: 'i;

            type Item = String;
            type Key = usize;

            fn len(&self) -> usize {
                0
            }

            fn iter(&self) -> Self::Iterator<'_> {
                Vec::new().into_iter()
            }
        }

        let a = OnToggleCallback::<(), MockModel>::default();
        let b = OnToggleCallback::<(), MockModel>::default();
        let c = a.clone();

        assert_eq!(a, c);
        assert_eq!(c, a);

        assert_ne!(a, b);
        assert_ne!(b, c);
    }
}
