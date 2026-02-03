use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardFooterProperties {
    /// Content rendered inside the Card Footer.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the Card Footer.
    #[prop_or_default]
    pub class: Classes,
    /// Sets the base component to render. Defaults to "div".
    #[prop_or(String::from("div"))]
    pub component: String,
}

#[function_component(CardFooter)]
pub fn body(props: &CardFooterProperties) -> Html {
    let class = classes!(props.class.clone(), "pf-v6-c-card__footer");
    html! {
        <@{props.component.clone()} {class}>
            {props.children.clone()}
        </@>
    }
}
