use wasm_bindgen::JsCast;
use web_sys::{Element, HtmlCollection, HtmlElement, HtmlInputElement, Node, NodeList};
use yew::NodeRef;

/// Focus an HTML input element.
///
/// The ref must point to an [`HtmlElement`], if it does not, the function does nothing.
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

/// Support working with [`web_sys::Element`]
pub trait ElementSupport {
    /// Call [`Element::contains`], or return `false` if this is not an [`Element`].
    fn contains(&self, target: Option<web_sys::EventTarget>) -> bool;
}

impl ElementSupport for NodeRef {
    fn contains(&self, target: Option<web_sys::EventTarget>) -> bool {
        let target = target.as_ref().and_then(|target| target.dyn_ref::<Node>());
        if let Some(element) = self.cast::<Element>() {
            element.contains(target)
        } else {
            false
        }
    }
}

/// Support working with [`web_sys::HtmlElement`]
pub trait HtmlElementSupport {
    /// Call [`HtmlElement::focus`] if this is an [`HtmlElement`].
    fn focus(&self);
}

impl HtmlElementSupport for NodeRef {
    fn focus(&self) {
        if let Some(input) = self.cast::<HtmlElement>() {
            let _ = input.focus();
        }
    }
}

/// Allow iterating over a [`NodeList`].
pub struct IterableNodeList<'a>(pub &'a NodeList);

impl<'a> IntoIterator for IterableNodeList<'a> {
    type Item = Node;
    type IntoIter = NodeListIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}

#[doc(hidden)]
pub struct NodeListIter<'a> {
    list: &'a NodeList,
    index: u32,
}

impl<'a> NodeListIter<'a> {
    pub fn new(list: &'a NodeList) -> Self {
        Self { list, index: 0 }
    }
}

impl<'a> Iterator for NodeListIter<'a> {
    type Item = Node;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.list.item(self.index);
        self.index += 1;
        next
    }
}

/// Allow iterating over an [`HtmlCollection`].
pub struct IterableHtmlCollection<'a>(pub &'a HtmlCollection);

impl<'a> IntoIterator for IterableHtmlCollection<'a> {
    type Item = Element;
    type IntoIter = HtmlCollectionIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter::new(self.0)
    }
}

#[doc(hidden)]
pub struct HtmlCollectionIter<'a> {
    list: &'a HtmlCollection,
    index: u32,
}

impl<'a> HtmlCollectionIter<'a> {
    pub fn new(list: &'a HtmlCollection) -> Self {
        Self { list, index: 0 }
    }
}

impl<'a> Iterator for HtmlCollectionIter<'a> {
    type Item = Element;

    fn next(&mut self) -> Option<Self::Item> {
        let next = self.list.item(self.index);
        self.index += 1;
        next
    }
}
