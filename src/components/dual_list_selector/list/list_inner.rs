//! The DualListSelectorList component.

use yew::prelude::*;

use super::{
    super::DualListSelectorItemRenderer as ItemRenderer,
    super::{OnOptionSelectArgsNoChosen, OnOptionSelectEvent},
    DualListSelectorListContext as Context, DualListSelectorListItem,
};

/// Acts as the container for DualListSelectorListItem sub-components.
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct DualListSelectorListProps {
    /// Content rendered inside the dual list selector list.
    #[prop_or_default]
    pub children: Html,
}

#[function_component(DualListSelectorList)]
pub fn list<T: ItemRenderer>(props: &DualListSelectorListProps) -> Html {
    let context = use_context::<Context<T>>().unwrap();
    html! {
        <ul class="pf-v6-c-dual-list-selector__list">
            if context.options.is_empty() {
                { props.children.clone() }
            } else {
                { for context.options.iter().enumerate().map(|(key, option)| {
                    let onoptionselect = {
                        let onoptionselect = context.onoptionselect.clone();
                        Callback::from(move |e: MouseEvent| {
                            let e: OnOptionSelectEvent = e.into();
                            let args = OnOptionSelectArgsNoChosen {
                                event: e,
                                index: key,
                            };
                            onoptionselect.emit(args)
                        })
                    };
                    let is_selected = context.selected_options.contains(&key);
                    html_nested! {
                        <DualListSelectorListItem key={key} {onoptionselect} {is_selected} disabled={context.disabled}>
                            { option.clone() }
                        </DualListSelectorListItem>
                    }
                })}
            }
        </ul>
    }
}
