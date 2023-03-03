use super::*;
use std::rc::Rc;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

/// Child of a toolbar component
#[derive(Clone, PartialEq)]
pub enum ToolbarChild {
    Item(Rc<<ToolbarItem as BaseComponent>::Properties>),
    Group(Rc<<ToolbarGroup as BaseComponent>::Properties>),
}

impl From<ToolbarItemProperties> for ToolbarChild {
    fn from(props: ToolbarItemProperties) -> Self {
        ToolbarChild::Item(Rc::new(props))
    }
}

impl From<ToolbarGroupProperties> for ToolbarChild {
    fn from(props: ToolbarGroupProperties) -> Self {
        ToolbarChild::Group(Rc::new(props))
    }
}

#[derive(PartialEq, Clone)]
pub struct ToolbarChildVariant {
    props: ToolbarChild,
}

impl<CHILD> From<VChild<CHILD>> for ToolbarChildVariant
where
    CHILD: BaseComponent,
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
            ToolbarChild::Item(props) => VComp::new::<ToolbarItem>(props, None).into(),
            ToolbarChild::Group(props) => VComp::new::<ToolbarGroup>(props, None).into(),
        }
    }
}
