use super::*;
use crate::prelude::*;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(PartialEq, Clone)]
pub struct MenuChildVariant {
    props: MenuChild,
}

impl<CHILD> From<VChild<CHILD>> for MenuChildVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<MenuChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for MenuChildVariant {
    fn into(self) -> Html {
        match self.props {
            MenuChild::Action(props) => VComp::new::<MenuAction>(props, None).into(),
            MenuChild::Link(props) => VComp::new::<MenuLink>(props, None).into(),
            MenuChild::Group(props) => VComp::new::<MenuGroup>(props, None).into(),
            MenuChild::Divider(props) => VComp::new::<ListDivider>(props, None).into(),
            MenuChild::Raw(props) => VComp::new::<Raw>(props, None).into(),
        }
    }
}
