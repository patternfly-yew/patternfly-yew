use std::fmt::Debug;
use yew::prelude::*;

use super::column::TableColumn;

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
    html! (
        <thead class="pf-v5-c-table__thead">

            <tr class="pf-v5-c-table__tr" role="row">

                if props.expandable {
                    <td class="pf-v5-c-table__td pf-v5-c-table__toggle" role="cell"></td>
                }

                { for props.children.iter() }

                if !props.hide_actions {
                    <td class="pf-v5-c-table__td"></td>
                }

            </tr>

        </thead>
    )
}
