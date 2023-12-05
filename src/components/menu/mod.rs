//! Menu components
mod child;
mod context;
mod group;
mod item;
mod toggle;
mod variant;

pub use child::*;
pub use context::*;
pub use group::*;
pub use item::*;
pub use toggle::*;
pub use variant::*;

use crate::prelude::{ElementSupport, IterableHtmlCollection};
use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlElement};
use yew::{html::ChildrenRenderer, prelude::*};
use yew_hooks::use_event_with_window;

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
}

#[function_component(Menu)]
pub fn menu(props: &MenuProperties) -> Html {
    let mut class = classes!("pf-v5-c-menu");

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
        >
            <div class="pf-v5-c-menu__content">
                <MenuList>{ props.children.clone() }</MenuList>
            </div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub(crate) struct MenuListProperties {
    pub(crate) children: ChildrenRenderer<MenuChildVariant>,
}

#[function_component(MenuList)]
pub(crate) fn menu_list(props: &MenuListProperties) -> Html {
    let r#ref = use_node_ref();

    {
        let r#ref = r#ref.clone();
        use_event_with_window("keydown", move |e: KeyboardEvent| {
            if !r#ref.contains(e.target()) {
                return;
            }

            handle_key(&r#ref, e);
        });
    }

    html!(
        <ul ref={r#ref} class="pf-v5-c-menu__list" role="menu">
            { for props.children.iter() }
        </ul>
    )
}

fn focusable_element(element: &HtmlElement) -> Option<HtmlElement> {
    element
        .query_selector("a, button, input")
        .ok()
        .flatten()
        .and_then(|element| element.dyn_into::<HtmlElement>().ok())
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

    if let Some(elements) = node
        .cast::<Element>()
        .map(|ele| ele.get_elements_by_tag_name("LI"))
    {
        let items = IterableHtmlCollection(&elements)
            .into_iter()
            .filter_map(|node| node.dyn_into::<HtmlElement>().ok())
            .filter(|element| {
                !element.class_list().contains("pf-m-disabled")
                    && !element.class_list().contains("pf-v5-c-divider")
            })
            .collect::<Vec<_>>();

        let len = items.len();

        let (index, _) = items
            .iter()
            .enumerate()
            .find(|(_, node)| focusable_element(node) == active)
            .unzip();

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
}
