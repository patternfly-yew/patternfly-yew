use crate::icon::Icon;
use std::fmt::Debug;
use yew::prelude::*;
use yew::virtual_dom::vnode::VNode::VComp;
use yew::virtual_dom::VChild;

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

#[derive(Clone, Debug, PartialEq)]
pub struct Span {
    pub cols: usize,
    pub content: Html,
}

impl Span {
    pub fn one(html: Html) -> Self {
        Self {
            cols: 1,
            content: html,
        }
    }
    pub fn max(html: Html) -> Self {
        Self {
            cols: usize::MAX,
            content: html,
        }
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
        html! {
            <tr role="row">
                { self.render_row(&entry.value)}
            </tr>
        }
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
            cells.push(html! {
                <td role="cell" colspan={cell.cols}>
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
                <td role="cell" data-label=label>
                    {cell}
                </td>
            });

            index += 1;
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

// table model

pub trait TableModel: Debug + Default + PartialEq + Clone {
    type Item: TableRenderer;

    /// Get the number of items
    fn len(&self) -> usize;
    /// Test if the entry is expanded
    fn is_expanded(&self, index: usize) -> bool;
    /// Set the expanded state of the entry
    fn set_expanded(&mut self, index: usize, state: bool) -> ShouldRender;
    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<Self::Item>) -> R;
}

#[derive(Clone, Debug, PartialEq)]
pub struct TableModelEntry<T> {
    pub value: T,
    expanded: bool,
    index: usize,
}

impl<T> TableModelEntry<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            expanded: false,
            index: 0,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    entries: Vec<TableModelEntry<T>>,
}

impl<T> Default for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    fn default() -> Self {
        Self { entries: vec![] }
    }
}

impl<T> From<Vec<T>> for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq,
{
    fn from(entries: Vec<T>) -> Self {
        let mut result = Vec::with_capacity(entries.len());

        let mut index = 0;
        for e in entries {
            result.push(TableModelEntry {
                value: e,
                index,
                expanded: false,
            });
            index += 1;
        }

        Self { entries: result }
    }
}

impl<T> TableModel for Vec<TableModelEntry<T>>
where
    T: TableRenderer + Clone + Debug + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.len()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.get(index).map(|e| e.expanded).unwrap_or(false)
    }

    fn set_expanded(&mut self, index: usize, state: bool) -> bool {
        if let Some(entry) = self.get_mut(index) {
            if entry.expanded != state {
                entry.expanded = state;
                true
            } else {
                false
            }
        } else {
            false
        }
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<T>) -> R,
    {
        let mut result = Vec::new();
        for entry in self {
            result.push(f(entry));
        }
        result
    }
}

impl<T> TableModel for SimpleTableModel<T>
where
    T: TableRenderer + Clone + Debug + PartialEq + 'static,
{
    type Item = T;

    fn len(&self) -> usize {
        self.entries.len()
    }

    fn is_expanded(&self, index: usize) -> bool {
        self.entries.is_expanded(index)
    }

    fn set_expanded(&mut self, index: usize, state: bool) -> bool {
        self.entries.set_expanded(index, state)
    }

    fn map<F, R>(&self, f: F) -> Vec<R>
    where
        F: Fn(&TableModelEntry<T>) -> R,
    {
        self.entries.map(f)
    }
}

// header

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct TableHeaderProps {
    #[prop_or_default]
    pub sticky: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<TableColumn>,
    #[prop_or_default]
    expandable: bool,
}

#[derive(Clone, PartialEq, Debug)]
pub struct TableHeader {
    props: TableHeaderProps,
}

impl Component for TableHeader {
    type Message = ();
    type Properties = TableHeaderProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <thead>

                <tr role="row">

                    { if self.props.expandable {
                        html!{<td></td>}
                    } else {
                        html!{}
                    }}

                    { for self.props.children.iter() }

                </tr>

            </thead>
        };
    }
}

// Column

#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProps {
    #[prop_or_default]
    pub label: String,
}

#[derive(Clone, Debug)]
pub struct TableColumn {
    props: TableColumnProps,
}

impl Component for TableColumn {
    type Message = ();
    type Properties = TableColumnProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <th role="columnheader">{ &self.props.label }</th>
        };
    }
}
