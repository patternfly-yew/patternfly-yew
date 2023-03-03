use super::variant::BreadcrumbChild;
use std::rc::Rc;
use yew::{prelude::*, virtual_dom::VComp};
use yew_nested_router::{components::Link, prelude::*};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbRouterItemProps<T: Target> {
    pub to: T,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    current: bool,
}

impl<T: Target> From<BreadcrumbRouterItemProps<T>> for BreadcrumbChild {
    fn from(props: BreadcrumbRouterItemProps<T>) -> Self {
        Self {
            creator: Rc::new(props),
        }
    }
}

impl<T: Target> super::variant::BreadcrumbItemCreator for BreadcrumbRouterItemProps<T> {
    fn create(mut self: Rc<Self>, current: bool) -> Html {
        let props = Rc::make_mut(&mut self);
        props.current = current;
        VComp::new::<BreadcrumbRouterItem<T>>(self, None).into()
    }
}

/// A breadcrumb item component based on [`yew_nested_router`].
#[function_component(BreadcrumbRouterItem)]
pub fn breadcrumb_router_item<T: Target>(props: &BreadcrumbRouterItemProps<T>) -> Html {
    let mut class = Classes::from("pf-c-breadcrumb__link");

    if props.current {
        class.push("pf-m-current");
    }

    html!(
        <Link<T>
            {class}
            target={props.to.clone()}
        >
            { for props.children.iter() }
        </Link<T>>
    )
}
