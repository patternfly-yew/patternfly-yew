use crate::prelude::*;
use std::rc::Rc;
use yew::prelude::*;

#[derive(Clone, PartialEq)]
pub enum MenuChild {
    Action(Rc<<MenuAction as BaseComponent>::Properties>),
    Link(Rc<<MenuLink as BaseComponent>::Properties>),
    Divider(Rc<<ListDivider as BaseComponent>::Properties>),
    Raw(Rc<<Raw as BaseComponent>::Properties>),
}

impl From<()> for MenuChild {
    fn from(_: ()) -> Self {
        MenuChild::Divider(Rc::new(()))
    }
}

impl From<MenuActionProperties> for MenuChild {
    fn from(props: MenuActionProperties) -> Self {
        MenuChild::Action(Rc::new(props))
    }
}

impl From<MenuLinkProperties> for MenuChild {
    fn from(props: MenuLinkProperties) -> Self {
        MenuChild::Link(Rc::new(props))
    }
}

impl From<ChildrenProperties> for MenuChild {
    fn from(props: ChildrenProperties) -> Self {
        MenuChild::Raw(Rc::new(props))
    }
}
