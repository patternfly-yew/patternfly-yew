use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardHeaderMainProperties {
    /// Contents rendered inside the Card Header Main
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the Card Header Main
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(CardHeaderMain)]
pub fn actions(props: &CardHeaderMainProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-card__header-main");
    html! {
        <div {class}>{props.children.clone()}</div>
    }
}
