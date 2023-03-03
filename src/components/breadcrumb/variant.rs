use std::rc::Rc;
use yew::{
    prelude::*,
    virtual_dom::{VChild, VComp},
};

use super::{BreadcrumbItem, BreadcrumbItemProperties};

pub trait BreadcrumbItemCreator {
    fn create(self: Rc<Self>, current: bool) -> Html;
}

impl BreadcrumbItemCreator for BreadcrumbItemProperties {
    fn create(mut self: Rc<Self>, current: bool) -> Html {
        let props = Rc::make_mut(&mut self);
        props.current = current;
        VComp::new::<BreadcrumbItem>(self, None).into()
    }
}

#[derive(Clone)]
pub struct BreadcrumbChild {
    pub creator: Rc<dyn BreadcrumbItemCreator>,
}

impl PartialEq for BreadcrumbChild {
    fn eq(&self, other: &Self) -> bool {
        Rc::ptr_eq(&self.creator, &other.creator)
    }
}

impl From<BreadcrumbItemProperties> for BreadcrumbChild {
    fn from(props: BreadcrumbItemProperties) -> Self {
        Self {
            creator: Rc::new(props),
        }
    }
}

#[derive(Clone, PartialEq)]
pub struct BreadcrumbItemVariant {
    props: BreadcrumbChild,
    current: bool,
}

impl BreadcrumbItemVariant {
    pub fn set_current(&mut self, current: bool) {
        self.current = current;
    }
}

impl<CHILD> From<VChild<CHILD>> for BreadcrumbItemVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<BreadcrumbChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
            current: false,
        }
    }
}

impl Into<Html> for BreadcrumbItemVariant {
    fn into(self) -> Html {
        self.props.creator.create(self.current)
    }
}
