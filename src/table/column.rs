use std::fmt::Debug;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProperties {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub center: bool,
}

/// The Table Column component.
///
/// ## Properties
///
/// Define by [`TableColumnProperties`].
#[function_component(TableColumn)]
pub fn table_column(props: &TableColumnProperties) -> Html {
    let class = match props.center {
        true => classes!("pf-m-center"),
        false => Classes::new(),
    };

    match &props.label {
        None => html! (<td></td>),
        Some(label) => html! (
            <th {class} role="columnheader">{ &label }</th>
        ),
    }
}
