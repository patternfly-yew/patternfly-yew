//! Tabs control
#[cfg(feature = "yew-nested-router")]
mod router;
mod simple;

#[cfg(feature = "yew-nested-router")]
pub use router::*;
pub use simple::*;

use yew::prelude::*;

/// Properties for [`TabContentBody`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TabContentBodyProperties {
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub padding: bool,
}

/// Tabs component body.
///
/// This is an optional sub-components used for styling the content of a tab.
///
/// ## Properties
///
/// Defined by [`TabContentBodyProperties`].
#[function_component(TabContentBody)]
pub fn tab_content_body(props: &TabContentBodyProperties) -> Html {
    let mut class = classes!("pf-c-tab-content__body");

    if props.padding {
        class.push(classes!("pf-m-padding"));
    }

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}
