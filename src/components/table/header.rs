use super::column::TableColumn;
use std::fmt::Debug;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeaderSortBy<K>
where
    K: Clone + Eq + 'static,
{
    pub index: K,
    pub asc: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableHeaderContext<K>
where
    K: Clone + Eq + 'static,
{
    pub sortby: Option<TableHeaderSortBy<K>>,
    pub onsort: Callback<Option<TableHeaderSortBy<K>>>,
}

/// Properties for [`TableHeader`]
#[derive(Debug, PartialEq, Clone, Properties)]
pub struct TableHeaderProperties<K>
where
    K: Clone + Eq + 'static,
{
    #[prop_or_default]
    pub sticky: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<TableColumn<K>>,
    #[prop_or_default]
    pub(crate) expandable: bool,
    #[prop_or_default]
    pub hide_actions: bool,
}

/// The Table Header component.
///
/// ## Properties
///
/// Defined by [`TableHeaderProperties`].
#[function_component(TableHeader)]
pub fn table_header<K>(props: &TableHeaderProperties<K>) -> Html
where
    K: Clone + Eq + 'static,
{
    let sortby: UseStateHandle<Option<TableHeaderSortBy<K>>> = use_state_eq(|| None);
    let onsort = use_callback(
        |val: Option<TableHeaderSortBy<K>>, sortby| {
            sortby.set(val);
        },
        sortby.clone(),
    );

    let table_header_context = TableHeaderContext {
        onsort,
        sortby: (*sortby).clone(),
    };

    html! (
        <thead class="pf-v5-c-table__thead">

            <tr class="pf-v5-c-table__tr" role="row">

                if props.expandable {
                    <td class="pf-v5-c-table__td pf-v5-c-table__toggle" role="cell"></td>
                }

                <ContextProvider<TableHeaderContext<K>> context={table_header_context}>
                    { for props.children.iter() }
                </ContextProvider<TableHeaderContext<K>>>

                if !props.hide_actions {
                    <td class="pf-v5-c-table__td"></td>
                }

            </tr>

        </thead>
    )
}
