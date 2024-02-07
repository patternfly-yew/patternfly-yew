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

use crate::ouia;
use crate::prelude::{Dropdown, ExtendClasses, Icon, MenuChildVariant, MenuToggleVariant};
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use std::rc::Rc;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VNode},
};

const OUIA: Ouia = ouia!("Table");

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

    /// OUIA Component id
    #[prop_or_else(|| OUIA.generated_id())]
    pub ouia_id: String,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
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

    pub fn are_columns_expandable(&self) -> bool {
        if let Some(header) = &self.header {
            header
                .props
                .children
                .iter()
                .any(|table_column| table_column.props.expandable)
        } else {
            false
        }
    }
}

/// Table component
///
/// > A **table** is used to display large data sets that can be easily laid out in a simple grid with column headers.
///
/// See: <https://www.patternfly.org/components/table/html>
///
/// ## Properties
///
/// Defined by [`TableProperties`].
///
/// ## Usage
///
/// The table component is a more complex component than most others. It is recommended to check
/// out the more complete examples in the quickstart project: <https://github.com/patternfly-yew/patternfly-yew-quickstart/tree/main/src/components/table>.
///
/// Summarizing it, you will need:
///
/// * A type defining the column/index (this can be an enum or a numeric like `usize`).
/// * A type defining an item/entry/row.
/// * Let the item type implement [`TableEntryRenderer`].
/// * Create a table state model (e.g. using [`MemoizedTableModel`]).
/// * Wire up the table state model (e.g. using [`use_table_data`]).
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[derive(Copy, Clone, Eq, PartialEq)]
/// enum Column { First, Second };
/// struct ExampleEntry { foo: String };
///
/// impl TableEntryRenderer<Column> for ExampleEntry {
///   fn render_cell(&self, context: CellContext<'_, Column>) -> Cell {
///     match context.column {
///       Column::First => html!(&self.foo).into(),
///       Column::Second => html!({self.foo.len()}).into(),
///     }
///   }
/// }
///
/// #[function_component(Example)]
/// fn example() -> Html {
///
///   let entries = use_memo((), |()| {
///       vec![
///           ExampleEntry { foo: "bar".into() },
///           ExampleEntry {
///               foo: "Much, much longer foo".into(),
///           },
///       ]
///   });
///
///   let (entries, _) = use_table_data(MemoizedTableModel::new(entries));
///
///   let header = html_nested! {
///     <TableHeader<Column>>
///       <TableColumn<Column> label="foo" index={Column::First} />
///       <TableColumn<Column> label="bar" index={Column::Second} />
///     </TableHeader<Column>>
///  };
///
///   html! (
///     <Table<Column, UseTableData<Column, MemoizedTableModel<ExampleEntry>>>
///       {header}
///       {entries}
///     />
///   )
/// }
/// ```
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

    let expandable_columns = use_memo(
        (props.header.clone(), props.mode.is_expandable()),
        |(header, expandable)| {
            if !expandable {
                return vec![];
            }

            match header {
                Some(header) => header
                    .props
                    .children
                    .iter()
                    .filter_map(|c| c.props.expandable.then(|| c.props.index.clone()))
                    .collect::<Vec<_>>(),
                None => vec![],
            }
        },
    );

    html! (
        <table
            id={&props.id}
            {class}
            role="grid"
            data-ouia-component-id={props.ouia_id.clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            if let Some(caption) = &props.caption {
                <caption class="pf-v5-c-table__caption">{caption}</caption>
            }
            { render_header(props) }
            { render_entries(props, &expandable_columns) }
        </table>
    )
}

fn render_header<C, M>(props: &TableProperties<C, M>) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let expandable = props.is_expandable() && !props.are_columns_expandable();
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

fn render_entries<C, M>(props: &TableProperties<C, M>, expandable_columns: &[C]) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    if props.is_expandable() {
        props
            .entries
            .iter()
            .map(|entry| render_expandable_entry(props, entry, expandable_columns))
            .collect()
    } else {
        html!(
            <tbody class="pf-v5-c-table__tbody" role="rowgroup"> {
                for props.entries.iter().map(|entry| {
                    let content = { render_row(props, &entry, |_| false)};
                    html!(
                        <tr class="pf-v5-c-table__tr" role="row" key={entry.key}>
                            {content}
                        </tr>
                    )}
                )
            } </tbody>
        )
    }
}

fn render_expandable_entry<C, M>(
    props: &TableProperties<C, M>,
    entry: TableModelEntry<M::Item, M::Key, C>,
    expandable_columns: &[C],
) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    let expansion = entry.expansion.clone();
    let expanded = expansion.is_some();

    let key = entry.key.clone();

    let mut cols = props
        .header
        .as_ref()
        .map_or(0, |header| header.props.children.len())
        + 1;

    let mut cells: Vec<Html> = Vec::with_capacity(cols);

    if expandable_columns.is_empty()
        && !entry
            .value
            .is_full_width_details()
            .unwrap_or(props.full_width_details)
    {
        cells.push(html! {<td class="pf-v5-c-table__td"></td>});
        cols -= 1;
    }

    let details = match expansion {
        Some(ExpansionState::Row) => entry.value.render_details(),
        Some(ExpansionState::Column(col)) => entry.value.render_column_details(&col),
        None => vec![],
    };

    for cell in details {
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
        props
            .onexpand
            .0
            .reform(move |_| (key.clone(), ExpansionState::Row))
    };

    let mut class = classes!("pf-v5-c-table__tr");

    if !expandable_columns.is_empty() && props.mode.is_expandable() {
        class.push(classes!("pf-v5-c-table__control-row"));
    }

    html! (
        <tbody {key} role="rowgroup" class={tbody_class}>
            <tr {class} role="row">

                // first column, the toggle

                if expandable_columns.is_empty() {
                    <TableRowToggle {expanded} {onclick} />
                }

                // then, the actual content

                { render_row(props, &entry, |column| expandable_columns.contains(column)) }
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

fn render_row<C, M, F>(
    props: &TableProperties<C, M>,
    entry: &TableModelEntry<'_, M::Item, M::Key, C>,
    expandable: F,
) -> Html
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
    F: Fn(&C) -> bool,
{
    let actions = entry.value.actions();

    let cols = props
        .header
        .iter()
        .flat_map(|header| header.props.children.iter());

    html!(<>
        { for cols.map(|column| {

            let index = column.props.index.clone();
            let expandable=  expandable(&index);


            // main cell content

            let cell = entry.value.render_cell(CellContext {
                column: &column.props.index,
            });

            // cell attributes

            let mut class = classes!("pf-v5-c-table__td");
            if cell.center {
                class.push(classes!("pf-m-center"))
            }
            class.extend_from(&cell.text_modifier);
            if expandable {
                class.push(classes!("pf-v5-c-table__compound-expansion-toggle"));
                match &entry.expansion {
                    Some(ExpansionState::Column(i)) if i == &index => {
                      class.push("pf-m-expanded");
                    }
                    _ => {},
                }
            }

            // data label

            let label = column.props.label.clone();

            // wrap with button when it's expandable
            let mut content = cell.content;
            if expandable {
                let key = entry.key.clone();
                let onclick = props.onexpand.0.reform(move |_| {
                    let toggle = ExpansionState::Column(index.clone());
                    (key.clone(), toggle)
                });

                content = html!(
                    <button class="pf-v5-c-table__button" {onclick}>
                        <span class="pf-v5-c-table__text">
                            { content }
                        </span>
                    </button>
                );
            }

            // render
            html!(
                <td {class} role="cell" data-label={label.unwrap_or_default()}>
                    { content }
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
