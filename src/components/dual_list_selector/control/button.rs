//! The DualListSelectorControl component.

use crate::components::{
    button::{Button, ButtonVariant},
    tooltip::Tooltip,
};
use yew::prelude::*;

/// Renders an individual control button for moving selected options between each
/// dual list selector pane.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorControlProps {
    /// Content to be displayed in a tooltip on hover of control.
    #[prop_or_default]
    pub tooltip: Option<AttrValue>,

    // TODO
    // /// Additional tooltip properties passed to the tooltip.
    // #[prop_or_default]
    // tooltip_props: anymap2::AnyMap,
    /// Flag indicating the control is disabled.
    #[prop_or(true)]
    pub disabled: bool,

    /// Content to be rendered in the dual list selector control.
    #[prop_or_default]
    pub children: Html,

    /// Callback fired when dual list selector control is selected.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

#[function_component(DualListSelectorControl)]
pub fn control(props: &DualListSelectorControlProps) -> Html {
    let button = html! {
        <Button
            disabled={props.disabled}
            variant={ButtonVariant::Plain}
            // TODO tabindex=-1
            onclick={props.onclick.clone()}
        >
            { props.children.clone() }
        </Button>
    };
    let inner = if let Some(text) = &props.tooltip {
        html! {
            <Tooltip text={text.to_string()}>
                { button }
            </Tooltip>
        }
    } else {
        button
    };
    html! {
        <div class="pf-v5-c-dual-list-selector__controls-item">
            { inner }
        </div>
    }
}
