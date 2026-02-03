use super::*;
use crate::hooks::id::use_random_id;
use yew::prelude::*;
use yew_nested_router::{components::Link, prelude::*};

// nav router item

/// Properties for [`NavRouterItem`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavRouterItemProperties<R>
where
    R: Target,
{
    #[prop_or_default]
    pub children: Html,
    pub to: R,

    #[prop_or_default]
    pub predicate: Option<Callback<R, bool>>,
}

/// A navigation item, using the Router.
#[function_component(NavRouterItem)]
pub fn nav_router_item<R>(props: &NavRouterItemProperties<R>) -> Html
where
    R: Target,
{
    let router = use_router().expect("Requires a Router or Nested router");

    let mut classes = Classes::from("pf-v6-c-nav__link");

    let active = router.is_active(&props.to, props.predicate.as_ref());

    let id = use_random_id();

    let expandable = use_expandable();
    use_effect_with(active, move |_| {
        if let Some(expandable) = expandable {
            expandable.state(*id, active)
        }
    });

    if active {
        classes.push("pf-m-current");
    }

    html! {
        <li class="pf-v6-c-nav__item">
            <Link<R> to={props.to.clone()} class={classes}>
                { props.children.clone() }
            </Link<R>>
        </li>
    }
}
