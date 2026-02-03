use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardSelectableActionsProperties {
    /// Content rendered inside the Card Selectable Action.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the selectable action.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CardSelectableActions)]
pub fn actions(props: &CardSelectableActionsProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-card__selectable-actions");
    html! {
        <div {class}>{props.children.clone()}</div>
    }
}
