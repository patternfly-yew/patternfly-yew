use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DescriptionList)]
pub fn dl(props: &DescriptionListProperties) -> Html {
    let classes = Classes::from("pf-c-description-list");

    html! (
        <dl class={classes}>
            { for props.children.iter() }
        </dl>
    )
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionGroupProperties {
    pub term: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DescriptionGroup)]
pub fn desc_group(props: &DescriptionGroupProperties) -> Html {
    html! (
        <div class="pf-c-description-list__group">
            <dt class="pf-c-description-list__term">{ &props.term }</dt>
            <dd class="pf-c-description-list__description">
                <div class="pf-c-description-list__text">
                    { for props.children.iter() }
                </div>
            </dd>
        </div>
    )
}
