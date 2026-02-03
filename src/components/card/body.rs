use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardBodyProperties {
    /// Content rendered inside the Card Body.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the Card Body.
    #[prop_or_default]
    pub class: Classes,
    /// Sets the base component ot render. Defaults to "div".
    #[prop_or(String::from("div"))]
    pub component: String,
    /// Enables the body Content to fill the height of the card
    #[prop_or(true)]
    pub filled: bool,
}

#[function_component(CardBody)]
pub fn body(props: &CardBodyProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-card__body");
    if !props.filled {
        class.push("pf-m-no-fill");
    }
    html! {
        <@{props.component.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}
