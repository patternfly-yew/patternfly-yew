use yew::prelude::*;
use yew::virtual_dom::{ApplyAttributeAs, Attributes, VNode, VTag};

/// Wrap an element in a div with the given class, preserving the
/// wrapped element's key property.
pub(crate) fn wrapper_div_with_attributes(
    child: VNode,
    attributes: &'static [(&'static str, &'static str, ApplyAttributeAs)],
) -> Html {
    let mut div = VTag::new("div");
    div.key = child.key().map(ToOwned::to_owned);
    div.attributes = Attributes::Static(attributes);
    div.add_child(child);
    VNode::VTag(div.into())
}
