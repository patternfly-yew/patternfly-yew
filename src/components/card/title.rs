use yew::prelude::*;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CardTitleProperties {
    /// Content rendered inside the Card Title.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the Card Title.
    #[prop_or_default]
    pub class: Classes,
    /// Sets the base component to render. Defaults to "div".
    #[prop_or(String::from("div"))]
    pub component: String,
}

#[function_component(CardTitle)]
pub fn card_title(props: &CardTitleProperties) -> Html {
    let class = classes!(props.class.clone(), "pf-v6-c-card__title-text");
    html! {
        <div class={"pf-v6-c-card__title"}>
            <@{props.component.clone()} {class}>
                {props.children.clone()}
            </@>
        </div>
    }
}
