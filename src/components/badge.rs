//! Badge
use yew::prelude::*;

/// Properties for [`Badge`]
#[derive(Clone, PartialEq, Properties)]
pub struct BadgeProperties {
    pub children: Children,
    #[prop_or_default]
    pub read: bool,
}

/// Badge component
///
/// > A **badge** is used to annotate other information like a label or an object name.
///
/// See: <https://www.patternfly.org/v4/components/badge>
///
/// ## Properties
///
/// Defined by [`BadgeProperties`].
#[function_component(Badge)]
pub fn badge(props: &BadgeProperties) -> Html {
    let mut classes = Classes::from("pf-v5-c-badge");

    if props.read {
        classes.push("pf-m-read");
    } else {
        classes.push("pf-m-unread");
    }

    html! {
        <span class={classes}>
            { for props.children.iter() }
        </span>
    }
}
