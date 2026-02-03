use yew::prelude::*;

use super::CardContext;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardExpandableContentProperties {
    /// Content rendered inside the expanded section.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the expanded section.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CardExpandableContent)]
pub fn expandable_content(props: &CardExpandableContentProperties) -> Html {
    let CardContext { expanded, .. } = use_context().expect("Could not get card context");
    if !expanded {
        return html!();
    }
    let class = classes!(props.class.clone(), "pf-v6-c-card__expandable-content");
    html! {
        <div {class}>
            {props.children.clone()}
        </div>
    }
}
