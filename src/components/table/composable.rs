use crate::prelude::{Button, ButtonVariant};
use yew::prelude::*;

use super::*;

const OUIA: Ouia = ouia!("Table");

/// Properties for [`ComposableTable`]
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ComposableTableProperties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub mode: TableMode,
    #[prop_or_default]
    pub sticky_header: bool,
    #[prop_or_default]
    pub grid: Option<TableGridMode>,
    #[prop_or(true)]
    pub borders: bool,
    #[prop_or_default]
    pub id: AttrValue,
    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

/// A table which that does not offer any of the type-safe utilities that [`Table`] offers.
/// Using this component means managing rows and columns manually.
/// It is recommended to use [`Table`] instead where possible.
/// A possible reason for using [`ComposableTable`] instead of [`Table`] is because state
/// needs to be saved per row.
///
/// ## Properties
///
/// Defined by [`ComposableTableProperties`].
#[function_component(ComposableTable)]
pub fn composable_table(props: &ComposableTableProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut class = classes!("pf-v6-c-table", props.class.clone());
    if props.sticky_header {
        class.push(classes!("pf-m-sticky-header"));
    }
    class.extend_from(&props.grid);

    if !props.borders {
        class.push(classes!("pf-m-no-border-rows"));
    }

    match props.mode {
        TableMode::Compact => {
            class.push(classes!("pf-m-compact"));
        }
        TableMode::CompactNoBorders => {
            class.push(classes!("pf-m-compact", "pf-m-no-border-rows"));
        }
        TableMode::CompactExpandable => {
            class.push(classes!("pf-m-compact"));
        }
        TableMode::Expandable => {
            class.push(classes!("pf-m-expandable"));
        }
        TableMode::Default => {}
    }

    html! {
        <table
            id={&props.id}
            {class}
            role="grid"
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            {props.children.clone()}
        </table>

    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CaptionProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(Caption)]
pub fn caption(props: &CaptionProperties) -> Html {
    html! {
        <caption class="pf-v6-c-table__caption">{props.children.clone()}</caption>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableBodyProperties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub expanded: bool,
}

#[function_component(TableBody)]
pub fn table_body(props: &TableBodyProperties) -> Html {
    let mut class = classes!("pf-v6-c-table__tbody", props.class.clone());
    if props.expanded {
        class.push("pf-m-expanded");
    }
    html! {
        <tbody {class} role="rowgroup">
            {props.children.clone()}
        </tbody>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableRowProperties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub expanded: bool,
    #[prop_or_default]
    pub control_row: bool,
}

#[function_component(TableRow)]
pub fn table_row(props: &TableRowProperties) -> Html {
    let mut class = classes!("pf-v6-c-table__tr", props.class.clone());
    if props.onclick.is_some() {
        class.push("pf-m-clickable");
    }
    if props.selected {
        class.push("pf-m-selected");
    }
    if props.expanded {
        class.push("pf-m-expanded");
    }
    if props.expandable {
        class.push("pf-v6-c-table__expandable-row");
    }
    if props.control_row {
        class.push("pf-v6-c-table__control-row");
    }
    html! {
        <tr class={class.clone()} role="row" onclick={props.onclick.clone()}>
            {props.children.clone()}
        </tr>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ExpandableRowContentProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ExpandableRowContent)]
pub fn expandable_row_content(props: &ExpandableRowContentProperties) -> Html {
    let class = classes!("pf-v6-c-table__expandable-row-content", props.class.clone());
    html! {
        <div {class}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct ExpandParams {
    pub r#type: ExpandType,
    pub expanded: bool,
    pub ontoggle: Callback<()>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExpandType {
    Row,
    Column,
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct TableDataProperties {
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub text_modifier: Option<TextModifier>,
    #[prop_or_default]
    pub expandable: Option<ExpandParams>,
    #[prop_or_default]
    pub data_label: Option<AttrValue>,
    #[prop_or_default]
    pub span_modifiers: Vec<SpanModifiers>,
    #[prop_or_default]
    pub colspan: Option<usize>,
    #[prop_or_default]
    pub action: bool,
}

#[function_component(TableData)]
pub fn table_data(props: &TableDataProperties) -> Html {
    let mut class = classes!("pf-v6-c-table__td", props.class.clone());
    if props.center {
        class.push(classes!("pf-m-center"))
    }
    if props.action {
        class.push("pf-v6-c-table__action");
    }
    class.extend_from(&props.text_modifier);
    class.extend_from(&props.span_modifiers);

    let mut content = props.children.clone();
    if let Some(expandable) = props.expandable.as_ref() {
        let onclick = {
            let ontoggle = expandable.ontoggle.clone();
            Callback::from(move |_| ontoggle.emit(()))
        };
        class.push(match expandable.r#type {
            ExpandType::Column => "pf-v6-c-table__compound-expansion-toggle",
            ExpandType::Row => "pf-v6-c-table__toggle",
        });
        content = match expandable.r#type {
            ExpandType::Column => {
                if expandable.expanded {
                    class.push("pf-m-expanded");
                }
                html! {
                    <button class="pf-v6-c-table__button" {onclick}>
                        <span class="pf-v6-c-table__text">
                            { content }
                        </span>
                    </button>
                }
            }
            ExpandType::Row => {
                let mut button_class = classes!();
                if expandable.expanded {
                    button_class.push("pf-m-expanded");
                }
                html! {
                    <Button
                        variant={ButtonVariant::Plain}
                        class={button_class}
                        {onclick}
                        aria_expanded={expandable.expanded.to_string()}
                    >
                        <div class="pf-v6-c-table__toggle-icon">
                            { Icon::AngleDown }
                        </div>
                    </Button>
                }
            }
        };
    }

    let colspan = props.colspan.as_ref().map(|cols| cols.to_string());
    html! {
        <td {class} role="cell" data-label={props.data_label.clone()} {colspan}>
            { content }
        </td>
    }
}
