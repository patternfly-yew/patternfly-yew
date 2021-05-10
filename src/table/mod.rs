mod column;
mod header;
mod model;

pub use column::*;
pub use header::*;
pub use model::*;

use crate::{icon::Icon, AsClasses, Dropdown, DropdownChildVariant, KebabToggle};
use std::fmt::Debug;
use yew::prelude::*;
use yew::virtual_dom::{vnode::VNode::VComp, VChild};

const LOG_TARGET: &'static str = "table";

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
pub struct TableProps<M>
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

#[derive(Clone, Debug)]
pub struct Table<M>
where
    M: TableModel + 'static,
{
    props: TableProps<M>,
    link: ComponentLink<Self>,
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
    fn as_classes(&self) -> Classes {
        match self {
            Self::Truncate => Classes::from("pf-m-truncate"),
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
    fn render(&self, column: ColumnIndex) -> Html;
    fn is_full_width_details(&self) -> Option<bool> {
        None
    }
    fn render_details(&self) -> Vec<Span> {
        vec![]
    }
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
    type Properties = TableProps<M>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut result = Self { props, link };
        result.sync_expandable();
        result
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        log::debug!(target: LOG_TARGET, "Update - msg: {:?}", msg);

        match msg {
            Msg::Collapse(idx) => self.set_expanded(idx, false),
            Msg::Expand(idx) => self.set_expanded(idx, true),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        log::debug!(target: LOG_TARGET, "Change: {:?}", props);
        if self.props != props {
            self.props = props;

            self.sync_expandable();

            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-table");

        if self
            .props
            .header
            .as_ref()
            .map_or(false, |header| header.props.sticky)
        {
            classes.push("pf-m-sticky-header");
        }

        match self.props.mode {
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

        return html! {
            <table class=classes role="grid">
                { self.render_caption() }
                { self.render_header() }
                { self.render_entries() }
            </table>
        };
    }
}

impl<M> Table<M>
where
    M: TableModel,
{
    fn sync_expandable(&mut self) {
        // sync down expandable state
        let expandable = self.is_expandable();
        if let Some(ref mut header) = self.props.header {
            header.props.expandable = expandable;
        }
    }

    fn render_caption(&self) -> Html {
        match &self.props.caption {
            Some(caption) => html! {
                <caption>{caption}</caption>
            },
            None => html! {},
        }
    }

    fn render_header(&self) -> Html {
        match &self.props.header {
            Some(header) => VComp(yew::virtual_dom::VComp::from(header.clone())),
            None => html! {},
        }
    }

    fn render_entries(&self) -> Vec<Html> {
        let expandable = self.is_expandable();

        let result = self.props.entries.map(|entry| match expandable {
            true => self.render_expandable_entry(&entry),
            false => self.render_normal_entry(&entry),
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

    fn render_normal_entry(&self, entry: &TableModelEntry<M::Item>) -> Html {
        return html! {
            <tr role="row">
                { self.render_row(&entry.value)}
            </tr>
        };
    }

    fn render_expandable_entry(&self, entry: &TableModelEntry<M::Item>) -> Html {
        let expanded = entry.expanded;
        let idx = entry.index;

        let onclick = match expanded {
            true => self.link.callback(move |_: MouseEvent| Msg::Collapse(idx)),
            false => self.link.callback(move |_: MouseEvent| Msg::Expand(idx)),
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

        let mut cols = self
            .props
            .header
            .as_ref()
            .map_or(0, |header| header.props.children.len())
            + 1;

        let mut cells: Vec<Html> = Vec::with_capacity(cols);

        if !entry
            .value
            .is_full_width_details()
            .unwrap_or(self.props.full_width_details)
        {
            cells.push(html! {<td></td>});
            cols -= 1;
        }

        for cell in entry.value.render_details() {
            let mut classes = Classes::new();
            classes = classes.extend(cell.modifiers.as_classes());
            cells.push(html! {
                <td class=classes role="cell" colspan={cell.cols}>
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
                <td colspan=cols></td>
            });
        }

        return html! {
            <tbody role="rowgroup" class=expanded_class.clone()>
                <tr role="row">
                    <td class="pf-c-table__toggle" role="cell">
                        <button class=classes onclick=onclick aria-expanded=aria_expanded>
                            <div class="pf-c-table__toggle_icon">
                                { if expanded { Icon::AngleDown } else { Icon::AngleRight }}
                            </div>
                        </button>
                    </td>

                    { self.render_row(&entry.value) }
                </tr>

                <tr class=("pf-c-table__expandable-row",expanded_class.clone()) role="row">
                    { cells }
                </tr>
            </tbody>
        };
    }

    fn set_expanded(&mut self, idx: usize, state: bool) -> ShouldRender {
        self.props.entries.set_expanded(idx, state)
    }

    fn render_row(&self, entry: &M::Item) -> Vec<Html> {
        let len = self
            .props
            .header
            .as_ref()
            .map_or(0, |header| header.props.children.len());

        let mut cells: Vec<Html> = Vec::with_capacity(len);

        let mut index = 0;
        for col in self
            .props
            .header
            .iter()
            .flat_map(|header| header.props.children.iter())
        {
            let cell = entry.render(ColumnIndex { index });
            let label = col.props.label;
            cells.push(html! {
                <td role="cell" data-label=label.unwrap_or_default()>
                    {cell}
                </td>
            });

            index += 1;
        }

        let actions = entry.actions();
        if !actions.is_empty() {
            cells.push(html! {
                <td class="pf-c-table__action">
                    <Dropdown
                        plain=true
                        toggle={html!{<KebabToggle/>}}
                        >
                        { actions }
                    </Dropdown>
                </td>
            });
        }

        cells
    }

    fn is_expandable(&self) -> bool {
        match self.props.mode {
            TableMode::Expandable | TableMode::CompactExpandable => true,
            _ => false,
        }
    }
}
