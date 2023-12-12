use web_sys::HtmlInputElement;
use yew::NodeRef;

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
