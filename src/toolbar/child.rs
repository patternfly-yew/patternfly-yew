use super::*;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Clone, PartialEq)]
pub enum ToolbarChild {
    Item(<ToolbarItem as Component>::Properties),
    Group(<ToolbarGroup as Component>::Properties),
}

impl From<ToolbarItemProps> for ToolbarChild {
    fn from(props: ToolbarItemProps) -> Self {
        ToolbarChild::Item(props)
    }
}

impl From<ToolbarGroupProps> for ToolbarChild {
    fn from(props: ToolbarGroupProps) -> Self {
        ToolbarChild::Group(props)
    }
}

#[derive(PartialEq, Clone)]
pub struct ToolbarChildVariant {
    props: ToolbarChild,
}

impl<CHILD> From<VChild<CHILD>> for ToolbarChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<ToolbarChild>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
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
