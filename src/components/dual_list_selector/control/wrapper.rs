//! The DualListSelectorControlsWrapper component

use yew::prelude::*;

/// Acts as the container for the DualListSelectorControl sub-components.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorControlsWrapperProps {
    /// Anything that can be rendered inside of the wrapper.
    pub children: Html,

    /// Additional classes added to the wrapper.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(DualListSelectorControlsWrapper)]
pub fn wrapper(props: &DualListSelectorControlsWrapperProps) -> Html {
    // TODO key handling
    let class = classes!["pf-v6-c-dual-list-selector__controls", props.class.clone()];
    html! {
        <div {class} tabindex=0 role="group">
            { props.children.clone() }
        </div>
    }
}
