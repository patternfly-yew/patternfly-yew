use crate::prelude::TableColumn;
use std::rc::Rc;
use yew::prelude::*;

/// Properties for [`TreeTableHeader`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TreeTableHeaderProperties<K>
where
    K: Clone + Eq + 'static,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<TableColumn<K>>,
}

/// The Table Header component.
///
/// ## Properties
///
/// Defined by [`TreeTableHeaderProperties`].
#[function_component(TreeTableHeader)]
pub fn tree_table_header<K>(props: &TreeTableHeaderProperties<K>) -> Html
where
    K: Clone + Eq + 'static,
{
    html! (
        <thead class="pf-v6-c-table__thead">

            <tr class="pf-v6-c-table__tr">

                { for props.children.iter().enumerate().map(|(n,mut c)| {
                    if n == 0 {
                        let props = Rc::make_mut(&mut c.props);
                        props.first_tree_column = true;
                    }
                    c
                }) }

                // this is for the actions
                <td></td>

            </tr>

        </thead>
    )
}
