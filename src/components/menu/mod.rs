//! Menu components
mod child;
mod context;
mod group;
mod item;
mod loading;
mod toggle;
mod variant;

pub use child::*;
pub use context::*;
pub use group::*;
pub use item::*;
pub use loading::*;
pub use toggle::*;
pub use variant::*;

use crate::ouia;
use crate::prelude::OuiaComponentType;
use crate::utils::{Ouia, OuiaSafe};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use web_tools::prelude::*;
use yew::{html::ChildrenRenderer, prelude::*};
use yew_hooks::use_event_with_window;

const OUIA: Ouia = ouia!("Menu");

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub r#ref: NodeRef,

    #[prop_or_default]
    pub scrollable: bool,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub children: ChildrenRenderer<MenuChildVariant>,

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

#[component]
pub fn Menu(props: &MenuProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut class = classes!("pf-v6-c-menu");

    if props.scrollable {
        class.push(classes!("pf-m-scrollable"));
    }

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    html!(
        <div
            ref={props.r#ref.clone()}
            id={props.id.clone()}
            style={&props.style}
            {class}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <div class="pf-v6-c-menu__content">
                <MenuList>{ props.children.clone() }</MenuList>
            </div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub(crate) struct MenuListProperties {
    pub(crate) children: ChildrenRenderer<MenuChildVariant>,
}

#[component]
pub(crate) fn MenuList(props: &MenuListProperties) -> Html {
    let r#ref = use_node_ref();

    {
        let r#ref = r#ref.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            handle_key(&r#ref, e);
        });
    }

    html!(
        <ul ref={r#ref} class="pf-v6-c-menu__list" role="menu">
            { for props.children.iter() }
        </ul>
    )
}

fn focusable_element(element: &HtmlElement) -> Option<HtmlElement> {
    element
        .query_selector("a, button, input")
        .ok()??
        .dyn_into::<HtmlElement>()
        .ok()
}

fn handle_key(node: &NodeRef, e: KeyboardEvent) {
    match e.key().as_str() {
        "Enter" => {
            if let Some(active) = gloo_utils::document()
                .active_element()
                .and_then(|element| element.dyn_into::<HtmlElement>().ok())
            {
                e.prevent_default();
                active.click();
            }
        }
        "ArrowUp" | "ArrowDown" => handle_arrows(node, e),
        _ => {}
    }
}

fn handle_arrows(node: &NodeRef, e: KeyboardEvent) {
    e.prevent_default();
    e.stop_immediate_propagation();

    let active = gloo_utils::document()
        .active_element()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok());

    let elements = match node
        .cast::<Element>()
        .map(|ele| ele.get_elements_by_tag_name("LI"))
    {
        Some(elements) => elements,
        None => return,
    };

    let items = IterableHtmlCollection(&elements)
        .into_iter()
        .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
        .filter(|element| {
            !element.class_list().contains("pf-m-disabled")
                && !element.class_list().contains("pf-v6-c-divider")
        })
        .collect::<Vec<_>>();

    let len = items.len();

    let index = items
        .iter()
        .position(|node| focusable_element(node) == active);

    let offset: isize = if e.key() == "ArrowDown" { 1 } else { -1 };

    let next_index = index
        // apply offset
        .map(|index| index as isize + offset)
        // handle overflow
        .map(|index| {
            if index < 0 {
                len.saturating_sub(1)
            } else if index as usize >= len {
                0
            } else {
                index as _
            }
        })
        // or default
        .unwrap_or_else(|| if offset > 0 { 0 } else { len.saturating_sub(1) });

    // get as node
    let next_node = items
        .get(next_index)
        .and_then(focusable_element)
        .and_then(|ele| ele.dyn_into::<HtmlElement>().ok());

    // apply
    if let Some(node) = &next_node {
        if let Some(active) = &active {
            active.set_tab_index(-1);
        }

        node.set_tab_index(0);
        let _ = node.focus();
    }
}
