use super::*;
use std::rc::Rc;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Clone, PartialEq)]
pub enum ToolbarChild {
    Item(Rc<<ToolbarItem as Component>::Properties>),
    Group(Rc<<ToolbarGroup as Component>::Properties>),
}

impl From<ToolbarItemProps> for ToolbarChild {
    fn from(props: ToolbarItemProps) -> Self {
        ToolbarChild::Item(Rc::new(props))
    }
}

impl From<ToolbarGroupProps> for ToolbarChild {
    fn from(props: ToolbarGroupProps) -> Self {
        ToolbarChild::Group(Rc::new(props))
    }
}

#[derive(PartialEq, Clone)]
pub struct ToolbarChildVariant {
    props: ToolbarChild,
}

impl<CHILD> From<VChild<CHILD>> for ToolbarChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<ToolbarChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for ToolbarChildVariant {
    fn into(self) -> Html {
        match self.props {
            ToolbarChild::Item(props) => {
                VComp::new::<ToolbarItem>(props, NodeRef::default(), None).into()
            }
            ToolbarChild::Group(props) => {
                VComp::new::<ToolbarGroup>(props, NodeRef::default(), None).into()
            }
        }
    }
}
