use yew::prelude::*;

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProperties {
    #[prop_or_default]
    pub children: Children,
}

/// The Description list component.
///
/// > A **description list** contains terms and their corresponding descriptions.
///
/// See: <https://www.patternfly.org/v4/components/description-list>
///
/// ## Properties
///
/// Defined by [`DescriptionListProperties`]. The component only has children.
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// pub fn example() -> Html {
///   html!(
///     <DescriptionList>
///       <DescriptionGroup term="42">{"Answer to the Ultimate Question of Life, The Universe, and Everything"}</DescriptionGroup>
///     </DescriptionList>
///   )
/// }
/// ```
#[function_component(DescriptionList)]
pub fn dl(props: &DescriptionListProperties) -> Html {
    let classes = Classes::from("pf-c-description-list");

    html! (
        <dl class={classes}>
            { for props.children.iter() }
        </dl>
    )
}

/// A [`DescriptionList`] entry.
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
