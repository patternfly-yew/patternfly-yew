use super::*;
use crate::utils::use_random_id;
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
    pub children: Children,
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

    let mut classes = Classes::from("pf-v5-c-nav__link");

    let active = router.is_active(&props.to, props.predicate.as_ref());

    let id = use_random_id();

    let expandable = use_expandable();
    use_effect_with_deps(
        move |_| {
            if let Some(expandable) = expandable {
                expandable.state(*id, active)
            }
        },
        active,
    );

    if active {
        classes.push("pf-m-current");
    }

    html! {
        <li class="pf-v5-c-nav__item">
            <Link<R> target={props.to.clone()} class={classes}>
                { for props.children.iter() }
            </Link<R>>
        </li>
    }
}
