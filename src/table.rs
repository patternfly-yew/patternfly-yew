use crate::icon::Icon;
use core::iter;
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
pub struct TableProps<T>
where
    T: Clone + Debug + PartialEq + TableRenderer + 'static,
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
    pub entries: Vec<T>,
}

#[derive(Clone, Debug)]
pub struct Table<T>
where
    T: Clone + Debug + PartialEq + TableRenderer + 'static,
{
    props: TableProps<T>,
    link: ComponentLink<Self>,
    expanded: Vec<bool>,
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

impl<T> Component for Table<T>
where
    T: Clone + Debug + PartialEq + TableRenderer + 'static,
{
    type Message = Msg;
    type Properties = TableProps<T>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let mut result = Self {
            props,
            link,
            expanded: Vec::new(),
        };
        result.sync_expandable();
        result.sync_expanded_state();
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
            self.sync_expanded_state();

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

impl<T> Table<T>
where
    T: Clone + Debug + PartialEq + TableRenderer + 'static,
{
    fn sync_expandable(&mut self) {
        // sync down expandable state
        let expandable = self.is_expandable();
        if let Some(ref mut header) = self.props.header {
            header.props.expandable = expandable;
        }
    }

    fn sync_expanded_state(&mut self) {
        let diff = self.props.entries.len() as isize - self.expanded.len() as isize;
        log::debug!(target: LOG_TARGET, "Diff: {}", diff);
        if diff < 0 {
            self.expanded.truncate(self.props.entries.len());
        } else if diff > 0 {
            self.expanded
                .extend(iter::repeat(false).take(diff as usize));
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
        let mut result: Vec<Html> = Vec::with_capacity(self.props.entries.len());

        let expandable = self.is_expandable();
        let mut idx = 0;

        for entry in &self.props.entries {
            result.push(match expandable {
                true => self.render_expandable_entry(idx, entry),
                false => self.render_normal_entry(idx, entry),
            });

            idx += 1;
        }

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

    fn render_normal_entry(&self, _: usize, entry: &T) -> Html {
        html! {
            <tr role="row">
                { self.render_row(&entry)}
            </tr>
        }
    }

    fn render_expandable_entry(&self, idx: usize, entry: &T) -> Html {
        let expanded = self.is_expanded(idx);

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
            .is_full_width_details()
            .unwrap_or(self.props.full_width_details)
        {
            cells.push(html! {<td></td>});
            cols -= 1;
        }

        for cell in entry.render_details() {
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

                    { self.render_row(&entry) }
                </tr>

                <tr class=("pf-c-table__expandable-row",expanded_class.clone()) role="row">
                    { cells }
                </tr>
            </tbody>
        };
    }

    fn is_expanded(&self, idx: usize) -> bool {
        self.expanded.get(idx).map(|is| *is).unwrap_or(false)
    }

    fn set_expanded(&mut self, idx: usize, state: bool) -> ShouldRender {
        let current = self.is_expanded(idx);
        if current != state {
            self.expanded[idx] = state;
            true
        } else {
            false
        }
    }

    fn render_row(&self, entry: &T) -> Vec<Html> {
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
