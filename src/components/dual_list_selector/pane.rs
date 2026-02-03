//! The DualListSelectorPane component.

use yew::prelude::*;

use super::{DualListSelectorItemRenderer, DualListSelectorList, DualListSelectorListWrapper};

/// Acts as the container for a list of options that are either available or chosen,
/// depending on the pane type (available or chosen).
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorPaneProps<T: DualListSelectorItemRenderer> {
    /// Additional classes applied to the dual list selector pane.
    #[prop_or_default]
    pub class: Classes,

    /// Options to list in the pane.
    #[prop_or_default]
    pub options: Vec<T>,

    /// Options currently selected in the pane.
    #[prop_or_default]
    pub selected_options: Vec<usize>,

    /// Callback for when an option is selected. Optionally used only when options prop is provided.
    pub onoptionselect: Callback<super::OnOptionSelectArgsNoChosen>,

    /// Title of the pane.
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Status to display above the pane.
    #[prop_or_default]
    pub status: Option<AttrValue>,

    /// Flag indicating if this pane is the chosen pane.
    #[prop_or_default]
    pub is_chosen: bool,

    /// Flag indicating whether the component is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// A dual list selector list to be rendered in the pane.
    #[prop_or_default]
    pub children: ChildrenWithProps<DualListSelectorList<T>>,
}

#[function_component(DualListSelectorPane)]
pub fn pane<T: DualListSelectorItemRenderer>(props: &DualListSelectorPaneProps<T>) -> Html {
    let mut class = classes!["pf-v6-c-dual-list-selector__pane", props.class.clone()];
    if props.is_chosen {
        class.push("pf-m-chosen")
    } else {
        class.push("pf-m-available")
    }
    let title = match &props.title {
        None => html! {},
        Some(title) => html! {
            <div class={classes!["pf-v6-c-dual-list-selector__header"]}>
                <div class={classes!["pf-v6-c-dual-list-selector__title"]}>
                    <div class={classes!["pf-v6-c-dual-list-selector__title-text"]}>
                        { title }
                    </div>
                </div>
            </div>
        },
    };
    let status = match &props.status {
        None => html! {},
        Some(status) => html! {
            <div class="pf-v6-c-dual-list-selector__status">
                <div class="pf-v6-c-dual-list-selector__status-text" id="dual-list-selector-basic-available-pane-status">
                    { status }
                </div>
            </div>
        },
    };
    html! {
        <div {class}>
            { title }
            { status }
            <DualListSelectorListWrapper<T>
                options={props.options.clone()}
                selected_options={props.selected_options.clone()}
                onoptionselect={props.onoptionselect.clone()}
                disabled={props.disabled}
            >
                { for props.children.iter() }
            </DualListSelectorListWrapper<T>>
        </div>
    }
}
