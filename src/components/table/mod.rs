mod cell;
mod column;
mod header;
mod model;
mod props;
mod render;

pub use cell::*;
pub use column::*;
pub use header::*;
pub use model::*;
pub use props::*;
pub use render::*;

use crate::prelude::{Dropdown, ExtendClasses, Icon, MenuChildVariant, MenuToggleVariant};
use std::rc::Rc;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VNode},
};

/// Properties for [`Table`]
#[derive(PartialEq, Clone, Properties)]
pub struct TableProperties<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub caption: Option<String>,
    #[prop_or_default]
    pub mode: TableMode,
    /// Borders or borderless.
    ///
    /// Defaults to borders being enabled.
    #[prop_or(true)]
    pub borders: bool,
    #[prop_or_default]
    pub header: Option<VChild<TableHeader<C>>>,
    #[prop_or_default]
    pub full_width_details: bool,
    pub entries: M,

    /// When to switch to grid mode
    #[prop_or_default]
    pub grid: Option<TableGridMode>,

    #[prop_or_default]
    pub onexpand: OnToggleCallback<C, M>,
}

impl<C, M> TableProperties<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    pub fn is_expandable(&self) -> bool {
        matches!(
            self.mode,
            TableMode::Expandable | TableMode::CompactExpandable
        )
    }
}

#[function_component(Table)]
pub fn table<C, M>(props: &TableProperties<C, M>) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let mut class = classes!("pf-v5-c-table");

    if props
        .header
        .as_ref()
        .map_or(false, |header| header.props.sticky)
    {
        class.push(classes!("pf-m-sticky-header"));
    }

    class.extend_from(&props.grid);

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

    if !props.borders {
        class.push(classes!("pf-m-no-border-rows"));
    }

    if !props.borders {
        class.push(classes!("pf-m-no-border-rows"));
    }

    html! (
        <table
            id={&props.id}
            {class}
            role="grid"
        >
            if let Some(caption) = &props.caption {
                <caption class="pf-v5-c-table__caption">{caption}</caption>
            }
            { render_header(props) }
            { render_entries(props) }
        </table>
    )
}

fn render_header<C, M>(props: &TableProperties<C, M>) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let expandable = props.is_expandable();
    match &props.header {
        Some(header) => {
            let mut header = header.clone();
            let props = Rc::make_mut(&mut header.props);
            props.expandable = expandable;
            VNode::VComp(yew::virtual_dom::VComp::from(header))
        }
        None => html!(),
    }
}

fn render_entries<C, M>(props: &TableProperties<C, M>) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    if props.is_expandable() {
        props
            .entries
            .iter()
            .map(|entry| render_expandable_entry(props, entry))
            .collect()
    } else {
        html!(
            <tbody class="pf-v5-c-table__tbody" role="rowgroup"> {
                for props.entries.iter().map(|entry| html!(
                    <tr class="pf-v5-c-table__tr" role="row" key={entry.key}>
                        { render_row(props, entry.value)}
                    </tr>
                ))
            } </tbody>
        )
    }
}

fn render_expandable_entry<C, M>(
    props: &TableProperties<C, M>,
    entry: TableModelEntry<M::Item, M::Key>,
) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let expanded = entry.expanded;
    let key = entry.key;

    let mut cols = props
        .header
        .as_ref()
        .map_or(0, |header| header.props.children.len())
        + 1;

    let mut cells: Vec<Html> = Vec::with_capacity(cols);

    if !entry
        .value
        .is_full_width_details()
        .unwrap_or(props.full_width_details)
    {
        cells.push(html! {<td class="pf-v5-c-table__td"></td>});
        cols -= 1;
    }

    for cell in entry.value.render_details() {
        let mut classes = classes!("pf-v5-c-table__td");
        classes.extend_from(&cell.modifiers);

        cells.push(html! {
            <td class={classes} role="cell" colspan={cell.cols.to_string()}>
                <div class="pf-v5-c-table__expandable-row-content">
                    { cell.content }
                </div>
            </td>
        });

        if cols > cell.cols {
            cols -= cell.cols;
        } else {
            cols = 0;
        }
        if cols == 0 {
            break;
        }
    }

    if cols > 0 {
        cells.push(html! (
            <td class="pf-v5-c-table__td" colspan={cols.to_string()}></td>
        ));
    }

    let mut tbody_class = classes!("pf-v5-c-table__tbody");
    let mut tr_class = classes!("pf-v5-c-table__tr", "pf-v5-c-table__expandable-row");

    if expanded {
        tbody_class.push(classes!("pf-m-expanded"));
        tr_class.push(classes!("pf-m-expanded"));
    }

    let onclick = {
        let key = key.clone();
        props.onexpand.0.reform(move |_| (key.clone(), !expanded))
    };

    html! (
        <tbody {key} role="rowgroup" class={tbody_class}>
            <tr class="pf-v5-c-table__tr" role="row">

                // first column, the toggle

                <TableRowToggle {expanded} {onclick} />

                // then, the actual content

                { render_row(props, entry.value) }
            </tr>

            // the expanded row details

            <tr class={tr_class} role="row">
                { cells }
            </tr>
        </tbody>
    )
}

#[derive(PartialEq, Properties)]
struct TableRowToggleProperties {
    expanded: bool,
    onclick: Callback<MouseEvent>,
}

#[function_component(TableRowToggle)]
fn table_row_toggle(props: &TableRowToggleProperties) -> Html {
    let aria_expanded = match props.expanded {
        true => "true",
        false => "false",
    };

    let mut toggle_class = classes!("pf-v5-c-button", "pf-m-plain");
    if props.expanded {
        toggle_class.push(classes!("pf-m-expanded"));
    }

    html!(
        <td class="pf-v5-c-table__td pf-v5-c-table__toggle" role="cell">
            <button
                class={toggle_class}
                onclick={props.onclick.clone()}
                aria-expanded={aria_expanded}
            >
                <div class="pf-v5-c-table__toggle-icon">
                    { Icon::AngleDown }
                </div>
            </button>
        </td>
    )
}

fn render_row<C, M>(props: &TableProperties<C, M>, entry: &M::Item) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let actions = entry.actions();

    let cols = props
        .header
        .iter()
        .flat_map(|header| header.props.children.iter());

    html!(<>
        { for cols.map(|column| {
            // main cell content
            
            let cell = entry.render_cell(CellContext {
                column: &column.props.index,
            });
        
            // cell attributes
            
            let mut class = classes!("pf-v5-c-table__td");
            if cell.center {
                class.push("pf-m-center")
            }
            class.extend_from(&cell.text_modifier);
    
            // data label
            
            let label = column.props.label.clone();
            
            // render
            
            html!(
                <td {class} role="cell" data-label={label.unwrap_or_default()}>
                    { cell.content }
                </td>
            )
        })}

        <RowActions {actions} />
    </>)
}

#[derive(PartialEq, Properties)]
struct RowActionsProperties {
    actions: Vec<MenuChildVariant>,
}

#[function_component(RowActions)]
fn row_actions(props: &RowActionsProperties) -> Html {
    html!(<>
        if !props.actions.is_empty() {
            <td class="pf-v5-c-table__td pf-v5-c-table__action" role="cell">
                <Dropdown
                    variant={MenuToggleVariant::Plain}
                    icon={Icon::EllipsisV}
                >
                    { props.actions.clone() }
                </Dropdown>
            </td>
        } 
    </>)
}

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub cols: usize,
    pub content: Html,
    pub modifiers: Vec<SpanModifiers>,
}

impl Span {
    pub fn one(html: Html) -> Self {
        Self {
            cols: 1,
            content: html,
            modifiers: Vec::new(),
        }
    }

    pub fn max(html: Html) -> Self {
        Self {
            cols: usize::MAX,
            content: html,
            modifiers: Vec::new(),
        }
    }

    pub fn truncate(mut self) -> Self {
        self.modifiers.push(SpanModifiers::Truncate);
        self
    }
}
