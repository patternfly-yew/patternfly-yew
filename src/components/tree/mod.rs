//! Tree table
//!
//! **NOTE:** This is in an experimental state.

mod header;
mod model;

pub use header::*;
pub use model::*;

use crate::{
    core::{AsClasses, ExtendClasses},
    prelude::CellContext,
    use_random_id,
};
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VChild};

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum TreeTableMode {
    #[default]
    Default,
    Compact,
    CompactNoBorders,
}

impl AsClasses for TreeTableMode {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Compact => {
                classes.push(classes!("pf-m-compact"));
            }
            Self::CompactNoBorders => {
                classes.push(classes!("pf-m-compact", "pf-m-no-border-rows"));
            }
            Self::Default => {}
        };
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct TreeTableProperties<T>
where
    T: TreeTableModel + PartialEq,
{
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub mode: TreeTableMode,

    pub header: VChild<TreeTableHeader>,

    pub model: Rc<T>,
}

#[function_component(TreeTable)]
pub fn tree_table<T>(props: &TreeTableProperties<T>) -> Html
where
    T: TreeTableModel + PartialEq + 'static,
{
    let mut class = classes!("pf-c-table", "pf-m-tree-view");

    class.extend_from(&props.mode);

    let headers = use_memo(
        |header| collect_columns(&header),
        props.header.props.clone(),
    );

    let content = use_memo(
        |(model, headers)| render_model(model, headers.clone()),
        (props.model.clone(), headers),
    );

    html!(
        <table
            id={&props.id}
            {class}
            role="treegrid"
        >

            { props.header.clone() }

            <tbody role="rowgroup">
            { (*content).clone() }
            </tbody>

        </table>
    )
}

#[derive(Clone, PartialEq, Eq)]
struct Column {
    label: Option<String>,
}

fn collect_columns(props: &TreeTableHeaderProperties) -> Vec<Column> {
    props
        .children
        .iter()
        .skip(1)
        .map(|c| Column {
            label: c.props.label.clone(),
        })
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct Visibility(bool);

impl Visibility {
    fn new() -> Self {
        Self(true)
    }

    fn nested(&self, level: bool) -> Self {
        if self.0 && level {
            Self(true)
        } else {
            Self(false)
        }
    }

    fn is_visible(&self) -> bool {
        self.0
    }
}

fn render_model<T>(model: &Rc<T>, headers: Rc<Vec<Column>>) -> Html
where
    T: TreeTableModel,
{
    render_nodes(1, model.children(), Visibility::new(), headers)
}

fn render_nodes(
    level: usize,
    nodes: Vec<Rc<dyn TreeNode>>,
    visibility: Visibility,
    headers: Rc<Vec<Column>>,
) -> Html {
    let size = nodes.len();
    html!(
        {
            for nodes.iter()
                .enumerate()
                .map(|(position,node) | html!(
                    <Row {visibility} {size} {position} {level} node={node.clone()} headers={headers.clone()}/>
                ))
        }
    )
}

#[derive(Properties)]
struct RowProperties {
    level: usize,
    size: usize,
    position: usize,
    node: Rc<dyn TreeNode>,
    visibility: Visibility,
    headers: Rc<Vec<Column>>,
}

impl PartialEq for RowProperties {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.node, &other.node)
            && self.level == other.level
            && self.size == other.size
            && self.position == other.position
            && self.visibility == other.visibility
            && self.headers == other.headers
    }
}

#[function_component(Row)]
fn row(props: &RowProperties) -> Html {
    let main = props.node.render_main();

    let mut main_class = classes!("pf-c-table__tree-view-title-cell");
    if main.center {
        main_class.push(classes!("pf-m-center"));
    }

    let expanded = use_state_eq(|| true);
    let ontoggle = {
        let expanded = expanded.clone();
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    let class = match *expanded {
        true => classes!("pf-m-expanded"),
        false => classes!(),
    };

    let children = props.node.children();

    html!(
        <>
            <tr
                {class}
                role="row"
                aria-level={ props.level.to_string() }
                aria-expanded={ (*expanded).to_string() }
                aria-setsize={ props.size.to_string() }
                aria-posinset={ props.position.to_string() }
                hidden={!props.visibility.is_visible()}
            >
                <th class={main_class}>
                    <MainCell has_children={!children.is_empty()} {ontoggle} expanded={*expanded} content={main.content}/>
                </th>
                { for props.headers.iter().enumerate().map(|(column, column_info)| {
                    let cell = props.node.render_cell(CellContext{column});
                    let class = match cell.center {
                        true => classes!("pf-m-center"),
                        false => Classes::new(),
                    };
                    html!(
                        <td {class} role="cell" data-label={column_info.label.clone()}>
                           {cell.content}
                        </td>
                    )
                }) }

                // cell for the actions
                <td></td>
            </tr>
            { render_nodes(props.level + 1, children, props.visibility.nested(*expanded), props.headers.clone()) }
        </>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct MainCellProperties {
    has_children: bool,
    expanded: bool,
    content: Html,
    ontoggle: Callback<()>,
}

#[function_component(MainCell)]
fn main_cell(props: &MainCellProperties) -> Html {
    let id_toggle = use_random_id();
    let id_label = use_random_id();

    let mut button_class = classes!("pf-c-button", "pf-m-plain");

    if props.expanded {
        button_class.push(classes!("pf-m-expanded"));
    }

    html!(
        <div class="pf-c-table__tree-view-main">
            if props.has_children {
                <span class="pf-c-table__toggle">
                    <button
                        class={button_class}
                        aria-labelledby={format!("{} {}", *id_label, *id_toggle)}
                        id={ *id_toggle }
                        aria-label="Details"
                        aria-expanded={ (props.expanded).to_string() }
                        onclick={props.ontoggle.reform(|_|())}
                    >
                        <div class="pf-c-table__toggle-icon">
                            <i class="fas fa-angle-down" aria-hidden="true"></i>
                        </div>
                    </button>
                </span>
            }
            <div class="pf-c-table__tree-view-text">
                <span
                    class="pf-c-table__text"
                    id={ *id_label }
                >
                    {props.content.clone()}
                </span>
            </div>
            // TODO: not sure why this is needed
            <span class="pf-c-table__tree-view-details-toggle">
                <button
                    class="pf-c-button pf-m-plain"
                    type="button"
                >
                    <span class="pf-c-table__details-toggle-icon">
                        <i class="fas fa-ellipsis-h" aria-hidden="true"></i>
                    </span>
                </button>
            </span>
        </div>
    )
}
