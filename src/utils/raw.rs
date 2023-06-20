use crate::prelude::ChildrenProperties;
use yew::prelude::*;

/// A general purpose "raw" component.
///
/// ## Idea
///
/// Some components require specific children. This is sometimes enforced using Rust's type system.
/// For example, the [`patternfly_yew::prelude::Card`] component requires either a
/// [`patternfly_yew::prelude::CardBody`] or [`patternfly_yew::prelude::Divider`]. And this is
/// enforced through Rust types. This ensures that components work as intended, but may be limiting
/// in some cases.
///
/// This component can be used (if a component supports it) as an alternative to the strongly typed
/// children, allowing to inject any component or element. Be aware, this might break things, but
/// allows for full control, like `unsafe`.
///
/// ## Children
///
/// The components accepts any and any number of components.
#[function_component(Raw)]
pub fn raw(props: &ChildrenProperties) -> Html {
    props.children.iter().collect()
}
