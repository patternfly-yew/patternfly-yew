use web_sys::HtmlInputElement;
use yew::prelude::*;

// TODO: remove this in the next version
pub use web_tools::{
    iter::{IterableHtmlCollection, IterableNodeList},
    optimistic::{
        OptimisticElement as ElementSupport, OptimisticHtmlElement as HtmlElementSupport,
    },
};

/// Focus an HTML input element.
///
/// The ref must point to an [`web_sys::HtmlElement`], if it does not, the function does nothing.
pub fn focus(node_ref: &NodeRef) {
    node_ref.focus();
}

/// Retrieve the value of an [`HtmlInputElement`].
///
/// The ref must point to an [`HtmlInputElement`], if it does not, the function will return [`None`].
pub fn value(node_ref: &NodeRef) -> Option<String> {
    node_ref
        .cast::<HtmlInputElement>()
        .map(|input| input.value())
}

#[hook]
pub(crate) fn use_document_href(href: &AttrValue) -> (NodeRef, AttrValue) {
    let node_ref = use_node_ref();
    let document = use_state_eq(|| None::<web_sys::Document>);
    {
        let document = document.clone();
        use_effect_with(node_ref.clone(), move |node_ref| {
            document.set(
                node_ref
                    .cast::<web_sys::Element>()
                    .and_then(|node| node.owner_document()),
            );
        });
    }
    let page_url = document.as_ref().and_then(|document| document.url().ok());
    (node_ref, resolve_fragment_href(href, page_url.as_deref()))
}

fn resolve_fragment_href(href: &AttrValue, page_url: Option<&str>) -> AttrValue {
    if href.starts_with('#')
        && let Some(page_url) = page_url
    {
        // A <base> element otherwise redirects fragment links away from the current page.
        let page_url = page_url.split('#').next().unwrap_or(page_url);
        return format!("{page_url}{href}").into();
    }
    href.clone()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn fragment_links_keep_the_current_route_and_query() {
        let page = Some("https://example.com/app/full/navigation/jump-links?mode=demo#old");
        assert_eq!(
            resolve_fragment_href(&"#section".into(), page),
            "https://example.com/app/full/navigation/jump-links?mode=demo#section"
        );
        assert_eq!(resolve_fragment_href(&"#section".into(), None), "#section");
        assert_eq!(
            resolve_fragment_href(&"#section".into(), Some("about:blank#old")),
            "about:blank#section"
        );
        assert_eq!(
            resolve_fragment_href(&"other#section".into(), page),
            "other#section"
        );
    }
}
