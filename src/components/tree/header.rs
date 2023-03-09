use crate::TableColumn;
use std::rc::Rc;
use yew::prelude::*;

/// Properties for [`TreeTableHeader`]
#[derive(Debug, PartialEq, Clone, Properties)]
pub struct TreeTableHeaderProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<TableColumn>,
}

/// The Table Header component.
///
/// ## Properties
///
/// Defined by [`TreeTableHeaderProperties`].
#[function_component(TreeTableHeader)]
pub fn tree_table_header(props: &TreeTableHeaderProperties) -> Html {
    html! (
        <thead>

            <tr role="row">

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
