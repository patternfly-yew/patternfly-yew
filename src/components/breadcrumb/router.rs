use super::variant::BreadcrumbChild;
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VComp};
use yew_nested_router::{components::Link, prelude::*};

/// Properties for [`BreadcrumbRouterItem`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbRouterItemProperties<T: Target> {
    pub to: T,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    current: bool,
}

impl<T: Target> From<BreadcrumbRouterItemProperties<T>> for BreadcrumbChild {
    fn from(props: BreadcrumbRouterItemProperties<T>) -> Self {
        Self {
            creator: Rc::new(props),
        }
    }
}

impl<T: Target> super::variant::BreadcrumbItemCreator for BreadcrumbRouterItemProperties<T> {
    fn create(mut self: Rc<Self>, current: bool) -> Html {
        let props = Rc::make_mut(&mut self);
        props.current = current;
        VComp::new::<BreadcrumbRouterItem<T>>(self, None).into()
    }
}

/// A breadcrumb item component based on [`yew_nested_router`].
#[function_component(BreadcrumbRouterItem)]
pub fn breadcrumb_router_item<T: Target>(props: &BreadcrumbRouterItemProperties<T>) -> Html {
    let mut class = Classes::from("pf-v6-c-breadcrumb__link");

    if props.current {
        class.push("pf-m-current");
    }

    html!(
        <Link<T>
            {class}
            to={props.to.clone()}
        >
            { props.children.clone() }
        </Link<T>>
    )
}
