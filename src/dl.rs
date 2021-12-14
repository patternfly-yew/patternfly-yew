use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DescriptionList)]
pub fn dl(props: &DescriptionListProps) -> Html {
    let classes = Classes::from("pf-c-description-list");

    return html! {
        <dl class={classes}>
            { for props.children.iter() }
        </dl>
    };
}

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionGroupProps {
    pub term: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(DescriptionGroup)]
pub fn desc_group(props: &DescriptionGroupProps) -> Html {
    html! {
        <div class="pf-c-description-list__group">
            <dt class="pf-c-description-list__term">{ &props.term }</dt>
            <dd class="pf-c-description-list__description">
                <div class="pf-c-description-list__text">
                    { for props.children.iter() }
                </div>
            </dd>
        </div>
    }
}
