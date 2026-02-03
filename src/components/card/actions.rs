use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardActionsProperties {
    /// Contents rendered inside the card action.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the action.
    #[prop_or_default]
    pub class: Classes,
    /// Flag indicating that the action have no offset.
    #[prop_or_default]
    pub has_no_offset: bool,
}

#[function_component(CardActions)]
pub fn actions(props: &CardActionsProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-card__actions");
    if props.has_no_offset {
        class.push("pf-m-no-offset");
    }
    html! {
        <div {class}>{props.children.clone()}</div>
    }
}
