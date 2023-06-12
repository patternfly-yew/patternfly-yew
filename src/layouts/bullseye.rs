//! Bullseye
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct BullseyeProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    /// Allows to disable wrapping children in the "item" div.
    ///
    /// According to the PatternFly documentation, this shouldn't make a difference. In practice,
    /// sometimes it does. Like when hosting a modal about dialog.
    pub plain: bool,
}

/// Bullseye layout
///
/// > Use a **Bullseye** layout to center content, both vertically and horizontally within a container.
///
/// See: <https://www.patternfly.org/v4/layouts/bullseye>
#[function_component(Bullseye)]
pub fn bullseye(props: &BullseyeProperties) -> Html {
    html! {
        <div class="pf-v5-l-bullseye">
            <div class="pf-v5-l-bullseye__item">{ for props.children.iter() }</div>
        </div>
    }
}
