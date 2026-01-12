use yew::prelude::*;
use yew::virtual_dom::{AttributeOrProperty, Attributes, VNode, VTag};

/// Wrap an element in another element with the given attributes,
/// preserving the wrapped element's key property.
pub(crate) fn wrapper_elt_with_attributes(
    child: VNode,
    element_name: &'static str,
    attributes: &'static [(&'static str, AttributeOrProperty)],
) -> Html {
    let mut elt = VTag::new(element_name);
    elt.key = child.key().map(ToOwned::to_owned);
    elt.attributes = Attributes::Static(attributes);
    elt.add_child(child);
    VNode::VTag(elt.into())
}

/// Wrap an element in a div with the given attributes, preserving the
/// wrapped element's key property.
pub(crate) fn wrapper_div_with_attributes(
    child: VNode,
    attributes: &'static [(&'static str, AttributeOrProperty)],
) -> Html {
    wrapper_elt_with_attributes(child, "div", attributes)
}
