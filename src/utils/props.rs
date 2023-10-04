use yew::prelude::*;

/// A properties structures which only has children.
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ChildrenProperties {
    #[prop_or_default]
    pub children: Html,
}
