/// The DualListSelectorItem component
use yew::prelude::*;

/// Creates an individual option that can be selected and moved between the dual list selector panes.
/// This is contained within the DualListSelectorList sub-component.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorItemProps {
    /// Additional classes applied to the dual list selector.
    #[prop_or_default]
    pub class: Classes,

    /// Flag indicating the list item is currently selected.
    #[prop_or_default]
    pub is_selected: bool,

    /// Callback fired when an option is selected.
    pub onoptionselect: Callback<MouseEvent>,

    /// Content rendered inside the dual list selector.
    pub children: Html,

    /// Flag indicating if the dual list selector is in a disabled state.
    #[prop_or_default]
    pub disabled: bool,
}

#[function_component(DualListSelectorListItem)]
pub fn list_item(props: &DualListSelectorItemProps) -> Html {
    let mut row_class = classes![
        "pf-v6-c-dual-list-selector__list-item-row",
        props.class.clone()
    ];
    if props.is_selected {
        row_class.push("pf-m-selected");
    }
    let mut item_class = classes!["pf-v6-c-dual-list-selector__list-item"];
    if props.disabled {
        item_class.push("pf-m-disabled")
    }
    html! {
        <li class={item_class} onclick={props.onoptionselect.clone()} tabindex="-1">
            <div class={row_class}>
                <span class="pf-v6-c-dual-list-selector__item">
                    <span class="pf-v6-c-dual-list-selector__item-main">
                        <span class="pf-v6-c-dual-list-selector__item-text">
                            { props.children.clone() }
                        </span>
                    </span>
                </span>
            </div>
        </li>
    }
}
