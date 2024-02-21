mod cell;
mod column;
mod composable;
mod header;
mod model;
mod props;
mod render;

pub use cell::*;
pub use column::*;
pub use composable::*;
pub use header::*;
pub use model::*;
pub use props::*;
pub use render::*;

use crate::ouia;
use crate::prelude::{Dropdown, ExtendClasses, Icon, MenuChildVariant, MenuToggleVariant};
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use yew::{prelude::*, virtual_dom::VChild};

const OUIA: Ouia = ouia!("Table");

/// Properties for [`Table`]
#[derive(PartialEq, Clone, Properties)]
pub struct TableProperties<C, M>
where
    C: Clone + Eq + 'static,
    M: PartialEq + TableModel<C> + 'static,
{
    #[prop_or_default]
    pub class: Classes,

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

    #[prop_or_default]
    pub onrowclick: Option<Callback<<M as TableModel<C>>::Item>>,
    /// Callback stating whether a given row is selected or not.
    #[prop_or_default]
    pub row_selected: Option<Callback<<M as TableModel<C>>::Item, bool>>,

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
/// If the table is too limiting (one example is wanting to save state per row) you may have to
/// use [`ComposableTable`].
/// However, it is recommended to use [`Table`] where possible.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[derive(Copy, Clone, Eq, PartialEq)]
/// enum Column { First, Second };
/// #[derive(Clone)]
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

    let expandable = props.is_expandable() && !props.are_columns_expandable();
    html!(
        <ComposableTable
            id={&props.id}
            class={props.class.clone()}
            sticky_header={props.header.as_ref().is_some_and(|header| header.props.sticky)}
            mode={props.mode}
            borders={props.borders}
            grid={props.grid}
            ouia_id={props.ouia_id.clone()}
            ouia_type={props.ouia_type}
            ouia_safe={props.ouia_safe}
        >
            if let Some(caption) = &props.caption {
                <Caption>{caption}</Caption>
            }
            if let Some(header) = props.header.clone() {
                <TableHeader<C> {expandable} ..(*header.props).clone() />
            }
            { render_entries(props, &expandable_columns) }
        </ComposableTable>
    )
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
        let row_click_cb = {
            let onrowclick = props.onrowclick.clone();
            Callback::from(move |entry| {
                if let Some(f) = onrowclick.as_ref() {
                    f.emit(entry)
                }
            })
        };
        html!(
            <TableBody> {
                for props.entries.iter().map(|entry| {
                    let selected = props.row_selected.as_ref().is_some_and(|f| f.emit(entry.value.clone()));
                    let content = { render_row(props, &entry, |_| false)};
                    let onclick = if props.onrowclick.is_some() {
                        let cb = row_click_cb.clone();
                        let val: M::Item = entry.value.clone();
                        Some(Callback::from(move |_| cb.emit(val.clone())))
                    } else {
                        None
                    };
                    html! {
                        <TableRow {onclick} {selected}>
                            {content}
                        </TableRow>
                    }
                })
            } </TableBody>
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
        cells.push(html! {<TableData />});
        cols -= 1;
    }

    let details = match expansion {
        Some(ExpansionState::Row) => entry.value.render_details(),
        Some(ExpansionState::Column(col)) => entry.value.render_column_details(&col),
        None => vec![],
    };

    for cell in details {
        cells.push(html! {
            <TableData span_modifiers={cell.modifiers.clone()} colspan={cell.cols}>
                <ExpandableRowContent>{ cell.content }</ExpandableRowContent>
            </TableData>
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
        cells.push(html!(
            <TableData colspan={cols}/>
        ));
    }

    let onclick = {
        let key = key.clone();
        props
            .onexpand
            .0
            .reform(move |_| (key.clone(), ExpansionState::Row))
    };

    html!(
        <TableBody {key} {expanded}>
            <TableRow control_row={!expandable_columns.is_empty() && props.mode.is_expandable()}>
                // first column, the toggle
                if expandable_columns.is_empty() {
                    <TableData expandable={ExpandParams {
                        r#type: ExpandType::Row,
                        expanded,
                        ontoggle: onclick,
                    }} />
                }
                // then, the actual content
                { render_row(props, &entry, |column| expandable_columns.contains(column)) }
            </TableRow>

            // the expanded row details
            <TableRow expandable=true {expanded}>
                { cells }
            </TableRow>
        </TableBody>
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
            let expandable = expandable(&index);

            // main cell content
            let cell = entry.value.render_cell(CellContext {
                column: &column.props.index,
            });

            let key = entry.key.clone();
            let expandable = expandable.then(|| ExpandParams {
                r#type: ExpandType::Column,
                ontoggle: props.onexpand.0.reform({
                    let index = index.clone();
                    move |_| {
                        let toggle = ExpansionState::Column(index.clone());
                        (key.clone(), toggle)
                    }
                }),
                expanded: entry.expansion == Some(ExpansionState::Column(index.clone())),
            });

            html!(
                <TableData
                    data_label={column.props.label.clone().map(AttrValue::from)}
                    {expandable}
                    center={cell.center}
                    text_modifier={cell.text_modifier}
                >
                    { cell.content.clone() }
                </TableData>
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
            <TableData action=true>
                <Dropdown
                    variant={MenuToggleVariant::Plain}
                    icon={Icon::EllipsisV}
                >
                    { props.actions.clone() }
                </Dropdown>
            </TableData>
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
