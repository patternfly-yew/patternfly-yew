//! Content wrapper
use yew::prelude::*;

/// Properties for [`Content`]
#[derive(Clone, PartialEq, Properties)]
pub struct ContentProperties {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub children: Html,
}

/// Content wrapper component
///
/// *NOTE:* In PatternFly, this is documented as "Text".
///
/// > A **text** component can wrap any static HTML content you want to place on your page to provide correct formatting when using standard HTML tags.
///
/// See: <https://www.patternfly.org/components/text/html>
///
/// ## Properties
///
/// Defined by [`ContentProperties`].
#[function_component(Content)]
pub fn content(props: &ContentProperties) -> Html {
    html! {
        <div
            class="pf-v6-c-content"
            id={&props.id}
        >
            { props.children.clone() }
        </div>
    }
}
