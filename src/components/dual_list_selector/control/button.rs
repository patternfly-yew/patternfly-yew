//! The DualListSelectorControl component.

use crate::{
    components::{
        button::{Button, ButtonVariant},
        tooltip::Tooltip,
    },
    prelude::TooltipProperties,
};
use yew::prelude::*;

/// Renders an individual control button for moving selected options between each
/// dual list selector pane.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorControlProps {
    /// Additional classes applied to the dual list selector control.
    #[prop_or_default]
    pub class: Classes,

    /// Content to be displayed in a tooltip on hover of control.
    #[prop_or_default]
    pub tooltip: Option<AttrValue>,

    /// Additional tooltip properties passed to the tooltip.
    #[prop_or_default]
    pub tooltip_props: Option<TooltipProperties>,

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
            tabindex={Some(-1)}
            onclick={props.onclick.clone()}
        >
            { props.children.clone() }
        </Button>
    };
    let inner = if let Some(text) = &props.tooltip {
        html! {
            if let Some(props) = &props.tooltip_props {
                <Tooltip text={text.to_string()} ..props.clone()>
                    { button }
                </Tooltip>
            } else {
                <Tooltip text={text.to_string()}>
                    { button }
                </Tooltip>
            }
        }
    } else {
        button
    };
    html! {
        <div class={classes!["pf-v6-c-dual-list-selector__controls-item", props.class.clone()]}>
            { inner }
        </div>
    }
}
