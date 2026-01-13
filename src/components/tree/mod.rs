//! Tree table
//!
//! **NOTE:** This is in an experimental state.

mod header;
mod model;

pub use header::*;
pub use model::*;

use crate::prelude::{AsClasses, CellContext, ExtendClasses, use_random_id};
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
pub struct TreeTableProperties<C, M>
where
    C: Clone + Eq + 'static,
    M: TreeTableModel<C> + PartialEq,
{
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub mode: TreeTableMode,

    pub header: VChild<TreeTableHeader<C>>,

    pub model: Rc<M>,

    #[prop_or(true)]
    pub default_expansion: bool,
}

#[function_component(TreeTable)]
pub fn tree_table<C, M>(props: &TreeTableProperties<C, M>) -> Html
where
    C: Clone + Eq + 'static,
    M: TreeTableModel<C> + PartialEq + 'static,
{
    let mut class = classes!("pf-v5-c-table", "pf-m-tree-view");

    class.extend_from(&props.mode);

    let headers = use_memo(props.header.props.clone(), |header| {
        collect_columns(&header)
    });

    let content = use_memo(
        (props.model.clone(), headers, props.default_expansion),
        |(model, headers, default_expansion)| {
            render_model(model, headers.clone(), *default_expansion)
        },
    );

    html!(
        <table
            id={&props.id}
            {class}
            role="treegrid"
        >

            { props.header.clone() }

            <tbody class="pf-v5-c-table__tbody">
                { (*content).clone() }
            </tbody>

        </table>
    )
}

#[derive(Clone, PartialEq, Eq)]
struct Column<C>
where
    C: Clone + Eq + 'static,
{
    label: Option<String>,
    index: C,
}

fn collect_columns<C>(props: &TreeTableHeaderProperties<C>) -> Vec<Column<C>>
where
    C: Clone + Eq + 'static,
{
    props
        .children
        .iter()
        .map(|c| Column {
            label: c.props.label.clone(),
            index: c.props.index.clone(),
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

fn render_model<C, M>(model: &Rc<M>, headers: Rc<Vec<Column<C>>>, default_expansion: bool) -> Html
where
    C: Clone + Eq + 'static,
    M: TreeTableModel<C>,
{
    render_nodes(
        1,
        model.children(),
        Visibility::new(),
        headers,
        default_expansion,
    )
}

fn render_nodes<C>(
    level: usize,
    nodes: Vec<Rc<dyn TreeNode<C>>>,
    visibility: Visibility,
    headers: Rc<Vec<Column<C>>>,
    default_expansion: bool,
) -> Html
where
    C: Clone + Eq + 'static,
{
    let size = nodes.len();
    html!(
        {
            for nodes.iter()
                .enumerate()
                .map(|(position,node) | html!(
                    <Row<C> {visibility} {size} {position} {level} node={node.clone()} headers={headers.clone()} {default_expansion}/>
                ))
        }
    )
}

#[derive(Properties)]
struct RowProperties<C>
where
    C: Clone + Eq + 'static,
{
    level: usize,
    size: usize,
    position: usize,
    node: Rc<dyn TreeNode<C>>,
    visibility: Visibility,
    headers: Rc<Vec<Column<C>>>,
    default_expansion: bool,
}

impl<C> PartialEq for RowProperties<C>
where
    C: Clone + Eq + 'static,
{
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
fn row<C>(props: &RowProperties<C>) -> Html
where
    C: Clone + Eq + 'static,
{
    let expanded = use_state_eq(|| props.default_expansion);

    let mut class = classes!("pf-v5-c-table__tr");

    if *expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    let children = props.node.children();

    html!(
        <>
            <tr
                {class}
                role="row"
                tabindex="0"
                aria-level={ props.level.to_string() }
                aria-expanded={ (*expanded).to_string() }
                aria-setsize={ props.size.to_string() }
                aria-posinset={ props.position.to_string() }
                hidden={!props.visibility.is_visible()}
            >

                { for props.headers.iter().enumerate().map(|(nr, column)| {

                    let cell = props.node.render_cell(CellContext{column: &column.index});
                    let mut class = match cell.center {
                        true => classes!("pf-m-center"),
                        false => Classes::new(),
                    };

                    match nr {
                        0 => {
                             let ontoggle = {
                                let expanded = expanded.clone();
                                Callback::from(move |_| {
                                    expanded.set(!*expanded);
                                })
                            };

                            class.push(classes!("pf-v5-c-table__th", "pf-v5-c-table__tree-view-title-cell"));
                            html!(
                                <th {class}>
                                    <MainCell has_children={!children.is_empty()} {ontoggle} expanded={*expanded}>
                                        { cell.content }
                                    </MainCell>
                                </th>
                            )
                        },
                        _ => {
                            class.push(classes!("pf-v5-c-table__td"));
                            html!(
                                <td {class} role="cell" data-label={column.label.clone()}>
                                   {cell.content}
                                </td>
                            )
                        },
                    }
                }) }

                // cell for the actions
                <td></td>
            </tr>
            if props.visibility.nested(*expanded).is_visible() {
                { render_nodes(props.level + 1, children, props.visibility.nested(*expanded), props.headers.clone(), props.default_expansion) }
            }
        </>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
struct MainCellProperties {
    children: Html,

    has_children: bool,
    expanded: bool,
    ontoggle: Callback<()>,
}

#[function_component(MainCell)]
fn main_cell(props: &MainCellProperties) -> Html {
    let id_toggle = use_random_id();
    let id_label = use_random_id();

    let mut button_class = classes!("pf-v5-c-button", "pf-m-plain");

    if props.expanded {
        button_class.push(classes!("pf-m-expanded"));
    }

    html!(
        <div class="pf-v5-c-table__tree-view-main">
            if props.has_children {
                <span class="pf-v5-c-table__toggle">
                    <button
                        type="button"
                        class={button_class}
                        aria-labelledby={format!("{} {}", *id_label, *id_toggle)}
                        id={ *id_toggle }
                        aria-label="Details"
                        aria-expanded={ (props.expanded).to_string() }
                        onclick={props.ontoggle.reform(|_|())}
                    >
                        <div class="pf-v5-c-table__toggle-icon">
                            <i class="fas fa-angle-down" aria-hidden="true"></i>
                        </div>
                    </button>
                </span>
            }
            <div class="pf-v5-c-table__tree-view-text">
                <span
                    class="pf-v5-c-table__text"
                    id={ *id_label }
                >
                    { props.children.clone() }
                </span>
            </div>
            // TODO: not sure why this is needed
            <span class="pf-v5-c-table__tree-view-details-toggle">
                <button
                    class="pf-v5-c-button pf-m-plain"
                    type="button"
                >
                    <span class="pf-v5-c-table__details-toggle-icon">
                        <i class="fas fa-ellipsis-h" aria-hidden="true"></i>
                    </span>
                </button>
            </span>
        </div>
    )
}
