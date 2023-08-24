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
    let mut class = classes!("pf-v5-c-tab-content__body");

    if props.padding {
        class.push(classes!("pf-m-padding"));
    }

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

/// Properties for [`TabContent`]
#[derive(PartialEq, Properties)]
pub struct TabContentProperties {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub hidden: bool,

    #[prop_or_default]
    pub children: Children,
}

/// Tabs component body.
///
/// > A **tab content** component should be used with the tabs component.
///
/// See: <https://www.patternfly.org/components/tab-content>
///
/// ## Properties
///
/// Defined by [`TabContentProperties`].
#[function_component(TabContent)]
pub fn tab_content(props: &TabContentProperties) -> Html {
    let class = Classes::from("pf-v5-c-tab-content");

    html!(
        <section
            id={props.id.clone()}
            {class}
            hidden={props.hidden}
            tabindex="0"
        >
            { for props.children.iter() }
        </section>
    )
}
