use std::fmt::Debug;
use yew::prelude::*;

#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProps {
    #[prop_or_default]
    pub label: Option<String>,
}

#[function_component(TableColumn)]
pub fn table_column(props: &TableColumnProps) -> Html {
    match &props.label {
        None => html! {},
        Some(label) => {
            html! {
                <th role="columnheader">{ &label }</th>
            }
        }
    }
}
