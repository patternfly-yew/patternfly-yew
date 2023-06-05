//! Badge
use yew::prelude::*;

/// Properties for [`Badge`]
#[derive(Clone, PartialEq, Properties)]
pub struct BadgeProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub read: bool,
    #[prop_or_default]
    pub screen_reader_text: AttrValue,
}

/// Badge component
///
/// > A **badge** is used to annotate other information like a label or an object name.
///
/// See: <https://pf5.patternfly.org/components/badge>
///
/// ## Properties
///
/// Defined by [`BadgeProperties`].
#[function_component(Badge)]
pub fn badge(props: &BadgeProperties) -> Html {
    let mut class = classes!("pf-v5-c-badge");
    if props.read {
        class.push("pf-m-read");
    } else {
        class.push("pf-m-unread");
    }
    class.extend(props.class.clone());
    html! {
        <span {class}>
            { props.children.clone() }
            if !props.screen_reader_text.is_empty() {
                <span class="pf-v5-u-screen-reader">{ props.screen_reader_text.clone() }</span>
            }
        </span>
    }
}
