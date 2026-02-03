//! The DualListSelcotrListWrapper component.

use yew::prelude::*;

use crate::components::dual_list_selector::OnOptionSelectArgsNoChosen;

use super::{super::DualListSelectorItemRenderer as ItemRenderer, DualListSelectorList};

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorListWrapperProps<T: ItemRenderer> {
    /// Additional classes applied to the dual list selector.
    #[prop_or_default]
    pub class: Classes,

    /// Options to list in the pane.
    #[prop_or_default]
    pub options: Vec<T>,

    /// Options currently selected in the pane.
    #[prop_or_default]
    pub selected_options: Vec<usize>,

    /// Callback for when an option is selected. Optionally used only when options prop is provided.
    pub onoptionselect: Callback<OnOptionSelectArgsNoChosen>,

    /// Flag indicating whether the component is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Anything that can be rendered inside of the list.
    #[prop_or_default]
    pub children: Children,
}

#[derive(Debug, Clone, PartialEq)]
pub struct DualListSelectorListContext<T: ItemRenderer> {
    pub options: Vec<T>,
    pub selected_options: Vec<usize>,
    pub onoptionselect: Callback<OnOptionSelectArgsNoChosen>,
    pub disabled: bool,
}

#[function_component(DualListSelectorListWrapper)]
pub fn wrapper<T: ItemRenderer>(props: &DualListSelectorListWrapperProps<T>) -> Html {
    let context = DualListSelectorListContext {
        options: props.options.clone(),
        selected_options: props.selected_options.clone(),
        onoptionselect: props.onoptionselect.clone(),
        disabled: props.disabled,
    };
    html! {
        <div class={classes!["pf-v6-c-dual-list-selector__menu", props.class.clone()]} tabindex=0>
            <ContextProvider<DualListSelectorListContext<T>> {context}>
                if !props.children.is_empty() {
                    { props.children.clone() }
                } else {
                    <DualListSelectorList<T>/>
                }
            </ContextProvider<DualListSelectorListContext<T>>>
        </div>
    }
}
