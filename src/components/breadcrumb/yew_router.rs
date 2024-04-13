//! [`yew_router`] breadcrumb links
use super::variant::BreadcrumbChild;
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VComp};
use ::yew_router::prelude::*;

/// Properties for [`BreadcrumbRouterItem`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbRouterItemProperties<R> where R: Routable + 'static {
    pub to: R,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    current: bool,
}

impl<R> From<BreadcrumbRouterItemProperties<R>> for BreadcrumbChild  where R: Routable + 'static {
    fn from(props: BreadcrumbRouterItemProperties<R>) -> Self {
        Self {
            creator: Rc::new(props),
        }
    }
}

impl<R> super::variant::BreadcrumbItemCreator for BreadcrumbRouterItemProperties<R> where R: Routable + 'static {
    fn create(mut self: Rc<Self>, current: bool) -> Html {
        let props = Rc::make_mut(&mut self);
        props.current = current;
        VComp::new::<BreadcrumbRouterItem<R>>(self, None).into()
    }
}

/// A breadcrumb item component based on [`yew_router`].
#[function_component(BreadcrumbRouterItem)]
pub fn breadcrumb_router_item<R>(props: &BreadcrumbRouterItemProperties<R>) -> Html where R: Routable + 'static {
    let mut classes = Classes::from("pf-v5-c-breadcrumb__link");

    if props.current {
        classes.push("pf-m-current");
    }

    html!(
        <Link<R>
            {classes}
            to={props.to.clone()}
        >
            { props.children.clone() }
        </Link<R>>
    )
}
