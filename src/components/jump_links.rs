//! Jump links

use crate::prelude::use_id;
use gloo_events::{EventListener, EventListenerOptions};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
struct ActiveJumpLink(UseStateHandle<Option<String>>);

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
/// The active item follows scrolling in the enclosing page main element, or
/// the owning document when used outside a page. Target `scroll-margin-top`
/// values are respected when determining the active section.
///
/// See: <https://www.patternfly.org/components/jump-links>
///
/// ## Properties
///
/// Defined by [`JumpLinksProperties`].
///
/// ## Children
///
/// Contains [`JumpLinksItem`] children. Use an item's `nested` property to add
/// a [`JumpLinksList`] of subsections beneath its link.
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
    let node_ref = use_node_ref();
    let active = use_state_eq(|| None::<String>);
    {
        let node_ref = node_ref.clone();
        let active = active.clone();
        use_effect_with(props.children.clone(), move |_| {
            let listeners = node_ref
                .cast::<web_sys::Element>()
                .and_then(|nav| observe_sections(nav, active));
            move || drop(listeners)
        });
    }

    let mut class = classes!("pf-v6-c-jump-links");
    if props.centered {
        class.push("pf-m-center");
    }
    if !props.horizontal {
        class.push("pf-m-vertical");
    }
    class.extend(props.class.clone());

    let aria_label = props
        .aria_label
        .clone()
        .or_else(|| props.label.as_ref().map(|_| AttrValue::from("Jump links")));

    html! {
        <ContextProvider<ActiveJumpLink> context={ActiveJumpLink(active)}>
        <nav ref={node_ref} {class} {id} aria-label={aria_label}>
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
        </ContextProvider<ActiveJumpLink>>
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

    /// Nested subsection lists, rendered beneath the link inside the list item.
    #[prop_or_default]
    pub nested: ChildrenWithProps<JumpLinksList>,

    /// Fallback active state before matching target sections can be tracked.
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
    let (node_ref, href) = crate::utils::use_document_href(&props.href);
    let context = use_context::<ActiveJumpLink>();
    let active = context
        .as_ref()
        .and_then(|context| context.0.as_ref())
        .map(|active| {
            props
                .href
                .rsplit_once('#')
                .is_some_and(|(_, id)| id == active)
        })
        .unwrap_or(props.active);
    let mut item_class = classes!("pf-v6-c-jump-links__item");
    if active {
        item_class.push("pf-m-current");
    }
    item_class.extend(props.class.clone());

    let aria_current = active.then_some(AttrValue::from("location"));

    html! {
        <li class={item_class} aria-current={aria_current}>
                <a
                    ref={node_ref}
                    class="pf-v6-c-jump-links__link"
                    {href}
                    onclick={props.onclick.clone()}
                >
                    <span class="pf-v6-c-jump-links__link-text">{ props.children.clone() }</span>
                </a>
            { for props.nested.iter() }
        </li>
    }
}

fn observe_sections(
    nav: web_sys::Element,
    active: UseStateHandle<Option<String>>,
) -> Option<(EventListener, EventListener)> {
    let document = nav.owner_document()?;
    let window = document.default_view()?;
    let page_url = document.url().ok()?;
    let links = nav.query_selector_all(".pf-v6-c-jump-links__link").ok()?;
    let sections = (0..links.length())
        .filter_map(|index| {
            use wasm_bindgen::JsCast;
            let link = links.item(index)?.dyn_into::<web_sys::Element>().ok()?;
            let href = link.get_attribute("href")?;
            let id = same_page_fragment(&href, &page_url)?;
            let decoded = js_sys::decode_uri_component(id).ok()?.as_string()?;
            Some((id.to_owned(), document.get_element_by_id(&decoded)?))
        })
        .collect::<Vec<_>>();
    if sections.is_empty() {
        active.set(None);
        return None;
    }
    let page_main = nav.closest(".pf-v6-c-page__main").ok().flatten();
    let scroller = page_main.clone().or_else(|| document.scrolling_element())?;
    let update = {
        let window = window.clone();
        Rc::new(move || {
            let top = page_main
                .as_ref()
                .map_or(0.0, |main| main.get_bounding_client_rect().top());
            let offsets = sections
                .iter()
                .map(|(_, section)| {
                    let margin = window
                        .get_computed_style(section)
                        .ok()
                        .flatten()
                        .and_then(|style| style.get_property_value("scroll-margin-top").ok())
                        .and_then(|value| value.trim_end_matches("px").parse::<f64>().ok())
                        .unwrap_or(0.0);
                    section.get_bounding_client_rect().top() - top - margin
                })
                .collect::<Vec<_>>();
            let at_end = scroller.scroll_top() > 0
                && scroller.scroll_top() + scroller.client_height() >= scroller.scroll_height() - 1;
            if let Some(index) = active_section_index(&offsets, at_end) {
                active.set(Some(sections[index].0.clone()));
            }
        })
    };
    update();
    let scroll = {
        let update = update.clone();
        EventListener::new_with_options(
            &window,
            "scroll",
            EventListenerOptions::run_in_capture_phase(),
            move |_| update(),
        )
    };
    let resize = EventListener::new(&window, "resize", move |_| update());
    Some((scroll, resize))
}

fn same_page_fragment<'a>(href: &'a str, page_url: &str) -> Option<&'a str> {
    let (page, id) = href.rsplit_once('#')?;
    (!id.is_empty() && (page.is_empty() || page == page_url.split('#').next()?)).then_some(id)
}

fn active_section_index(offsets: &[f64], at_end: bool) -> Option<usize> {
    if offsets.is_empty() {
        None
    } else if at_end {
        Some(offsets.len() - 1)
    } else {
        Some(offsets.iter().rposition(|top| *top <= 1.0).unwrap_or(0))
    }
}

#[cfg(test)]
mod tracking_tests {
    use super::*;

    #[test]
    fn active_section_tracks_scrolling_and_the_last_section_at_the_bottom() {
        assert_eq!(active_section_index(&[], false), None);
        assert_eq!(active_section_index(&[16.0, 200.0, 400.0], false), Some(0));
        assert_eq!(active_section_index(&[-200.0, 0.5, 200.0], false), Some(1));
        assert_eq!(active_section_index(&[-400.0, -200.0, 0.0], false), Some(2));
        assert_eq!(active_section_index(&[-200.0, -16.0, 180.0], true), Some(2));
    }

    #[test]
    fn only_tracks_fragment_targets_on_the_same_page() {
        let page = "https://example.com/app/demo?mode=test#old";
        assert_eq!(same_page_fragment("#section", page), Some("section"));
        assert_eq!(
            same_page_fragment("https://example.com/app/demo?mode=test#section", page),
            Some("section")
        );
        assert_eq!(same_page_fragment("other#section", page), None);
        assert_eq!(same_page_fragment("#", page), None);
    }
}

#[cfg(all(test, not(target_family = "wasm")))]
mod tests {
    use super::*;
    use crate::test::render_html;

    #[test]
    fn vertical_links_support_nested_lists() {
        let html = render_html(|| {
            html! {
                <JumpLinks id="sections" aria_label="Sections">
                    <JumpLinksItem href="#parent" active=true nested={html_nested! {
                        <JumpLinksList>
                            <JumpLinksItem href="#child">{"Child"}</JumpLinksItem>
                        </JumpLinksList>
                    }}>
                        {"Parent"}
                    </JumpLinksItem>
                </JumpLinks>
            }
        });
        assert!(html.contains("pf-m-vertical"));
        assert!(html.contains("aria-current=\"location\""));
        assert!(html.contains("<a href=\"#parent\" class=\"pf-v6-c-jump-links__link\""));
        assert!(html.contains("</a><ul role=\"list\" class=\"pf-v6-c-jump-links__list\""));
        assert!(html.contains("href=\"#child\""));
    }

    #[test]
    fn horizontal_links_use_default_patternfly_orientation() {
        let html = render_html(|| {
            html! {
                <JumpLinks id="sections" horizontal=true />
            }
        });
        assert!(!html.contains("pf-m-vertical"));
        assert!(!html.contains("pf-m-horizontal"));
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
