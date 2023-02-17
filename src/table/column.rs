use std::fmt::Debug;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProperties {
    #[prop_or_default]
    pub label: Option<String>,
}

/// The Table Column component.
///
/// ## Properties
///
/// Define by [`TableColumnProperties`].
#[function_component(TableColumn)]
pub fn table_column(props: &TableColumnProperties) -> Html {
    match &props.label {
        None => html! (<td></td>),
        Some(label) => html! (
            <th role="columnheader">{ &label }</th>
        ),
    }
}
