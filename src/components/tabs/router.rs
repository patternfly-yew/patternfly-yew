use crate::{AsClasses, Inset};
use std::fmt::Debug;
use yew::prelude::*;
use yew_nested_router::{components::Link, prelude::*};

// tab router

/// Properties for [`TabsRouter`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabsRouterProperties<T>
where
    T: Target + 'static,
{
    #[prop_or_default]
    pub r#box: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub filled: bool,

    #[prop_or_default]
    pub inset: Option<Inset>,

    #[prop_or_default]
    pub children: ChildrenWithProps<TabRouterItem<T>>,
}

#[function_component(TabsRouter)]
pub fn tabs_router<T>(props: &TabsRouterProperties<T>) -> Html
where
    T: Target,
{
    let mut classes = classes!("pf-v5-c-tabs");

    if props.r#box {
        classes.push("pf-m-box");
    }

    if props.vertical {
        classes.push("pf-m-vertical");
    }

    if props.filled {
        classes.push("pf-m-fill");
    }

    if let Some(inset) = &props.inset {
        inset.extend_classes(&mut classes);
    }

    html! (
        <div class={classes}>
            <ul class="pf-v5-c-tabs__list">
                { for props.children.iter() }
            </ul>
        </div>
    )
}

// tab router item

/// Properties for [`TabRouterItem`]
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TabRouterItemProperties<T>
where
    T: Target,
{
    /// The tab label
    pub label: String,
    /// The switch this item references to
    pub to: T,
}

#[function_component(TabRouterItem)]
pub fn tab_router_item<T>(props: &TabRouterItemProperties<T>) -> Html
where
    T: Target,
{
    let router = use_router::<T>().expect("Must be used below a Router or Nested component");

    let mut classes = Classes::from("pf-v5-c-tabs__item");

    if router.is_same(&props.to) {
        classes.push("pf-m-current");
    }

    html! (
        <li class={classes}>
            <Link<T> element="button" class="pf-v5-c-tabs__link" target={props.to.clone()}>
                <span class="pf-v5-c-tabs__item-text"> { &props.label } </span>
            </Link<T>>
        </li>
    )
}
