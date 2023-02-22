mod column;
mod header;
mod model;

pub use column::*;
pub use header::*;
pub use model::*;

use crate::{icon::Icon, AsClasses, Dropdown, DropdownChildVariant, KebabToggle};
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use yew::prelude::*;
use yew::virtual_dom::{vnode::VNode::VComp, VChild};

const LOG_TARGET: &str = "table";

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub enum TableMode {
    Default,
    Compact,
    CompactNoBorders,
    CompactExpandable,
    Expandable,
}

impl Default for TableMode {
    fn default() -> Self {
        Self::Default
    }
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct TableProperties<M>
where
    M: TableModel + 'static,
{
    #[prop_or_default]
    pub caption: Option<String>,
    #[prop_or_default]
    pub mode: TableMode,
    #[prop_or_default]
    pub header: Option<VChild<TableHeader>>,
    #[prop_or_default]
    pub full_width_details: bool,
    #[prop_or_default]
    pub entries: M,
}

/// The Table component.
///
/// > A **table** is used to display large data sets that can be easily laid out in a simple grid with column headers.
///
/// See: https://www.patternfly.org/v4/components/table
///
/// ## Properties
///
/// Defined by [`TableProperties`].
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[derive(Clone, PartialEq)]
/// pub struct Item {
///     string: String,
///     number: u32,
/// }
///
/// impl TableRenderer for Item {
///     fn render(&self, column: ColumnIndex) -> Html {
///         match column.index {
///             0 => html!(&self.string),
///             1 => html!(&self.number),
///             _ => html!(),
///         }
///     }
/// }
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let header = html_nested!(
///     <TableHeader>
///       <TableColumn/>
///       <TableColumn label="Number"/>
///     </TableHeader>
///   );
///   let entries = SharedTableModel::new(vec![
///     Item { string: "Foo".to_string(), number: 23 },
///     Item { string: "Bar".to_string(), number: 42 },
///     Item { string: "Baz".to_string(), number: 0 },
///   ]);
///   html!(
///     <Table<SharedTableModel<Item>> {header} {entries} />
///   )
/// }
/// ```
///
#[derive(Clone)]
pub struct Table<M>
where
    M: TableModel + 'static,
{
    _marker: PhantomData<M>,
}

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ColumnIndex {
    pub index: usize,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanModifiers {
    Truncate,
}

impl AsClasses for SpanModifiers {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Truncate => classes.push("pf-m-truncate"),
        }
    }
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

/// Render table entries
pub trait TableRenderer {
    /// Render the requested column.
    fn render(&self, column: ColumnIndex) -> Html;

    /// Control if the details section is spans the full width.
    fn is_full_width_details(&self) -> Option<bool> {
        None
    }

    /// Render the details section.
    ///
    /// Defaults to not having details.
    fn render_details(&self) -> Vec<Span> {
        vec![]
    }

    /// Render the row actions.
    ///
    /// Defaults to no actions.
    fn actions(&self) -> Vec<DropdownChildVariant> {
        vec![]
    }
}

#[derive(Clone, Debug)]
pub enum Msg {
    Collapse(usize),
    Expand(usize),
}

impl<M> Component for Table<M>
where
    M: TableModel + 'static,
{
    type Message = Msg;
    type Properties = TableProperties<M>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        log::debug!(target: LOG_TARGET, "Update - msg: {:?}", msg);

        match msg {
            Msg::Collapse(idx) => self.set_expanded(ctx, idx, false),
            Msg::Expand(idx) => self.set_expanded(ctx, idx, true),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-table");

        if ctx
            .props()
            .header
            .as_ref()
            .map_or(false, |header| header.props.sticky)
        {
            classes.push("pf-m-sticky-header");
        }

        match ctx.props().mode {
            TableMode::Compact => {
                classes.push("pf-m-compact");
            }
            TableMode::CompactNoBorders => {
                classes.push("pf-m-compact");
                classes.push("pf-m-no-border-rows");
            }
            TableMode::CompactExpandable => {
                classes.push("pf-m-compact");
                classes.push("pf-m-expandable");
            }
            TableMode::Expandable => {
                classes.push("pf-m-expandable");
            }
            TableMode::Default => {}
        };

        html! (
            <table class={classes} role="grid">
                { self.render_caption(ctx) }
                { self.render_header(ctx) }
                { self.render_entries(ctx) }
            </table>
        )
    }
}

impl<M> Table<M>
where
    M: TableModel,
{
    fn render_caption(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().caption {
            Some(caption) => html! {
                <caption>{caption}</caption>
            },
            None => html! {},
        }
    }

    fn render_header(&self, ctx: &Context<Self>) -> Html {
        match &ctx.props().header {
            Some(header) => {
                let mut header = header.clone();
                let props = Rc::make_mut(&mut header.props);
                props.expandable = self.is_expandable(ctx);
                VComp(yew::virtual_dom::VComp::from(header))
            }
            None => html! {},
        }
    }

    fn render_entries(&self, ctx: &Context<Self>) -> Vec<Html> {
        let expandable = self.is_expandable(ctx);

        let result = ctx.props().entries.map(|entry| match expandable {
            true => self.render_expandable_entry(ctx, entry),
            false => self.render_normal_entry(ctx, entry),
        });

        if expandable {
            result
        } else {
            vec![html! {
                <tbody role="rowgroup">
                    {result}
                </tbody>
            }]
        }
    }

    fn render_normal_entry(&self, ctx: &Context<Self>, entry: &TableModelEntry<M::Item>) -> Html {
        html!(
            <tr role="row">
                { self.render_row(ctx, &entry.value)}
            </tr>
        )
    }

    fn render_expandable_entry(
        &self,
        ctx: &Context<Self>,
        entry: &TableModelEntry<M::Item>,
    ) -> Html {
        let expanded = entry.expanded;
        let idx = entry.index;

        let onclick = match expanded {
            true => ctx.link().callback(move |_: MouseEvent| Msg::Collapse(idx)),
            false => ctx.link().callback(move |_: MouseEvent| Msg::Expand(idx)),
        };

        let mut classes = Classes::from("pf-c-button");
        classes.push("pf-m-plain");
        if expanded {
            classes.push("pf-m-expanded");
        }

        let aria_expanded = match expanded {
            true => "true",
            false => "false",
        };

        let mut expanded_class = Classes::new();
        if expanded {
            expanded_class.push("pf-m-expanded");
        }

        let mut cols = ctx
            .props()
            .header
            .as_ref()
            .map_or(0, |header| header.props.children.len())
            + 1;

        let mut cells: Vec<Html> = Vec::with_capacity(cols);

        if !entry
            .value
            .is_full_width_details()
            .unwrap_or(ctx.props().full_width_details)
        {
            cells.push(html! {<td></td>});
            cols -= 1;
        }

        for cell in entry.value.render_details() {
            let mut classes = Classes::new();
            classes.extend(cell.modifiers.as_classes());
            cells.push(html! {
                <td class={classes} role="cell" colspan={cell.cols.to_string()}>
                    <div class="pf-c-table__expandable-row-content">
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
            cells.push(html! {
                <td colspan={cols.to_string()}></td>
            });
        }

        let mut tr_classes = classes!("pf-c-table__expandable-row");
        tr_classes.extend(expanded_class.clone());

        html! (
            <tbody role="rowgroup" class={expanded_class}>
                <tr role="row">
                    <td class="pf-c-table__toggle" role="cell">
                        <button class={classes} onclick={onclick} aria-expanded={aria_expanded}>
                            <div class="pf-c-table__toggle_icon">
                                { if expanded { Icon::AngleDown } else { Icon::AngleRight }}
                            </div>
                        </button>
                    </td>

                    { self.render_row(ctx, &entry.value) }
                </tr>

                <tr class={tr_classes} role="row">
                    { cells }
                </tr>
            </tbody>
        )
    }

    fn set_expanded(&mut self, ctx: &Context<Self>, idx: usize, state: bool) -> bool {
        ctx.props().entries.set_expanded(idx, state)
    }

    fn render_row(&self, ctx: &Context<Self>, entry: &M::Item) -> Vec<Html> {
        let len = ctx
            .props()
            .header
            .as_ref()
            .map_or(0, |header| header.props.children.len());

        let mut cells: Vec<Html> = Vec::with_capacity(len);

        for (index, col) in ctx
            .props()
            .header
            .iter()
            .flat_map(|header| header.props.children.iter())
            .enumerate()
        {
            let cell = entry.render(ColumnIndex { index });
            let label = col.props.label.clone();
            cells.push(html! {
                <td role="cell" data-label={label.unwrap_or_default()}>
                    {cell}
                </td>
            });
        }

        let toggle = html! {<KebabToggle/>};
        let actions = entry.actions();
        if !actions.is_empty() {
            cells.push(html! {
                <td class="pf-c-table__action">
                    <Dropdown
                        plain=true
                        toggle={toggle}
                        >
                        { actions }
                    </Dropdown>
                </td>
            });
        }

        cells
    }

    fn is_expandable(&self, ctx: &Context<Self>) -> bool {
        matches!(
            ctx.props().mode,
            TableMode::Expandable | TableMode::CompactExpandable
        )
    }
}
