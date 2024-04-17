//! yew_router nav links
use ::yew_router::prelude::*;
use yew::prelude::*;

/// Properties for [`NavRouterLink`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavRouterLinkProperties<R>
where
    R: Routable + 'static,
{
    #[prop_or_default]
    pub children: Html,
    pub to: R,
}

/// A navigation link, using Yew Router.
#[function_component(NavRouterLink)]
pub fn nav_yew_router_link<R>(
    NavRouterLinkProperties { children, to }: &NavRouterLinkProperties<R>,
) -> Html
where
    R: Routable + 'static,
{
    let mut classes = Classes::from("pf-v5-c-nav__link");
    let current_route: R = use_route().expect("Requires a Router");
    let is_active = &current_route == to;

    if is_active {
        classes.push("pf-m-current");
    }

    html! {
        <li class="pf-v5-c-nav__item">
            <Link<R> to={to.clone()} {classes}>
                { children.clone() }
            </Link<R>>
        </li>
    }
}
