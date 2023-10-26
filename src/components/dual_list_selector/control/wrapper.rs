//! The DualListSelectorControlsWrapper component

use yew::prelude::*;

/// Acts as the container for the DualListSelectorControl sub-components.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorControlsWrapperProps {
    /// Anything that can be rendered inside of the wrapper.
    pub children: Html,
}

#[function_component(DualListSelectorControlsWrapper)]
pub fn wrapper(props: &DualListSelectorControlsWrapperProps) -> Html {
    // TODO key handling
    html! {
        <div class="pf-v5-c-dual-list-selector__controls" tabindex=0 role="group">
            { props.children.clone() }
        </div>
    }
}
