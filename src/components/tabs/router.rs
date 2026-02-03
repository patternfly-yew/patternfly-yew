use super::TabTitle;
use crate::ouia;
use crate::prelude::{AsClasses, Inset, OuiaComponentType};
use crate::utils::{Ouia, OuiaSafe};
use std::fmt::Debug;
use yew::prelude::*;
use yew_nested_router::{components::Link, prelude::*};

const OUIA_TABS: Ouia = ouia!("Tabs");

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

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA_TABS.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

#[function_component(TabsRouter)]
pub fn tabs_router<T>(props: &TabsRouterProperties<T>) -> Html
where
    T: Target,
{
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA_TABS.generated_id())
    });
    let mut classes = classes!("pf-v6-c-tabs");

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
        <div
            class={classes}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <ul class="pf-v6-c-tabs__list">
                { for props.children.iter() }
            </ul>
        </div>
    )
}

// tab router item
const OUIA_ITEM: Ouia = ouia!("TabsItem");

/// Properties for [`TabRouterItem`]
#[derive(Properties, Clone, PartialEq)]
pub struct TabRouterItemProperties<T>
where
    T: Target,
{
    /// The tab title
    pub title: TabTitle,
    /// The switch this item references to
    pub to: T,
    /// If tab is disabled
    #[prop_or(false)]
    pub disabled: bool,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA_ITEM.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

#[function_component(TabRouterItem)]
pub fn tab_router_item<T>(props: &TabRouterItemProperties<T>) -> Html
where
    T: Target,
{
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA_ITEM.generated_id())
    });
    let router = use_router::<T>().expect("Must be used below a Router or Nested component");

    let mut classes = Classes::from("pf-v6-c-tabs__item");

    if router.is_same(&props.to) {
        classes.push("pf-m-current");
    }

    let mut link_classes = Classes::from("pf-v6-c-tabs__link");

    if props.disabled {
        link_classes.push("pf-m-disabled");
    }

    html! (
        <li
            class={classes}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <Link<T> element="button" class={link_classes} to={props.to.clone()}>
                <span class="pf-v6-c-tabs__item-text"> { props.title.clone() } </span>
            </Link<T>>
        </li>
    )
}
