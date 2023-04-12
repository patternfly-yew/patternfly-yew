use web_sys::{HtmlElement, HtmlInputElement};
use yew::NodeRef;

/// Focus an HTML input element.
///
/// The ref must point to an [`HtmlElement`], if it does not, the function does nothing.
pub fn focus(node_ref: &NodeRef) {
    if let Some(input) = node_ref.cast::<HtmlElement>() {
        let _ = input.focus();
    }
}

/// Retrieve the value of an [`HtmlInputElement`].
///
/// The ref must point to an [`HtmlInputElement`], if it does not, the function will return [`None`].
pub fn value(node_ref: &NodeRef) -> Option<String> {
    node_ref
        .cast::<HtmlInputElement>()
        .map(|input| input.value())
}
