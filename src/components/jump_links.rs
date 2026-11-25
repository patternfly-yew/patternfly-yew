//! Jump links

use crate::prelude::use_id;
use yew::prelude::*;

/// Properties for [`JumpLinks`].
#[derive(Clone, PartialEq, Properties)]
pub struct JumpLinksProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<JumpLinksItem>,

    /// Accessible label for the nav element.
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,

    /// Optional visible label rendered above the list.
    #[prop_or_default]
    pub label: Option<Html>,

    /// Whether the list is displayed vertically (default) or horizontally.
    #[prop_or_default]
    pub horizontal: bool,

    /// Whether the list is centered.
    #[prop_or_default]
    pub centered: bool,

    /// Additional classes for the nav element.
    #[prop_or_default]
    pub class: Classes,

    /// Optional id for the nav element.
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// Jump links component
///
/// > **Jump links** are a navigational aid that allow users to skip to specific
/// > sections of a page. They are typically placed in a sidebar and link to
/// > anchored sections via fragment identifiers.
///
/// See: <https://www.patternfly.org/components/jump-links>
///
/// ## Properties
///
/// Defined by [`JumpLinksProperties`].
///
/// ## Children
///
/// Contains [`JumpLinksItem`] children, optionally nested inside [`JumpLinksList`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(
///         <JumpLinks aria_label="Jump to sections" label={html!({ "On this page" })}>
///             <JumpLinksItem href="#section-1">{ "Section 1" }</JumpLinksItem>
///             <JumpLinksItem href="#section-2" active=true>{ "Section 2" }</JumpLinksItem>
///             <JumpLinksItem href="#section-3">{ "Section 3" }</JumpLinksItem>
///         </JumpLinks>
///     )
/// }
/// ```
#[function_component(JumpLinks)]
pub fn jump_links(props: &JumpLinksProperties) -> Html {
    let id = use_id(props.id.clone());

    let mut class = classes!("pf-v6-c-jump-links");
    if props.centered {
        class.push("pf-m-center");
    }
    if props.horizontal {
        class.push("pf-m-horizontal");
    }
    class.extend(props.class.clone());

    let aria_label = props
        .aria_label
        .clone()
        .or_else(|| props.label.as_ref().map(|_| AttrValue::from("Jump links")));

    html! {
        <nav {class} {id} aria-label={aria_label}>
            <div class="pf-v6-c-jump-links__main">
                if let Some(label) = &props.label {
                    <div class="pf-v6-c-jump-links__header">
                        <div class="pf-v6-c-jump-links__label">{ label.clone() }</div>
                    </div>
                }
                <ul class="pf-v6-c-jump-links__list" role="list">
                    { for props.children.iter() }
                </ul>
            </div>
        </nav>
    }
}

/// Properties for [`JumpLinksItem`].
#[derive(Clone, PartialEq, Properties)]
pub struct JumpLinksItemProperties {
    /// The `href` fragment identifier to jump to, e.g. `#section-1`.
    pub href: AttrValue,

    /// The link text.
    #[prop_or_default]
    pub children: Html,

    /// Whether this item is the currently active section.
    #[prop_or_default]
    pub active: bool,

    /// Click handler.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,

    /// Additional classes for the list item.
    #[prop_or_default]
    pub class: Classes,
}

/// A single jump link item.
///
/// ## Properties
///
/// Defined by [`JumpLinksItemProperties`].
#[function_component(JumpLinksItem)]
pub fn jump_links_item(props: &JumpLinksItemProperties) -> Html {
    let mut item_class = classes!("pf-v6-c-jump-links__item");
    if props.active {
        item_class.push("pf-m-current");
    }
    item_class.extend(props.class.clone());

    let aria_current = props.active.then_some(AttrValue::from("location"));

    html! {
        <li class={item_class} aria-current={aria_current}>
            <span class="pf-v6-c-jump-links__link">
                <a
                    class="pf-v6-c-button pf-m-link"
                    href={props.href.clone()}
                    onclick={props.onclick.clone()}
                >
                    <span class="pf-v6-c-jump-links__link-text">{ props.children.clone() }</span>
                </a>
            </span>
        </li>
    }
}

/// Properties for [`JumpLinksList`].
#[derive(Clone, PartialEq, Properties)]
pub struct JumpLinksListProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<JumpLinksItem>,

    /// Additional classes for the list.
    #[prop_or_default]
    pub class: Classes,
}

/// A nested list of jump links, used for subsections.
///
/// ## Properties
///
/// Defined by [`JumpLinksListProperties`].
#[function_component(JumpLinksList)]
pub fn jump_links_list(props: &JumpLinksListProperties) -> Html {
    let mut class = classes!("pf-v6-c-jump-links__list");
    class.extend(props.class.clone());

    html! {
        <ul {class} role="list">
            { for props.children.iter() }
        </ul>
    }
}
