//! Hint

use yew::prelude::*;
use yew::virtual_dom::VNode;

/// Properties for [`Hint`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintProperties {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub actions: Option<VNode>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// Hint component
///
/// > A **hint** is in-app messaging that provides a one-step reminder, explanation, or call to action for a page or modal.
///
/// See: <https://www.patternfly.org/v4/components/hint>
///
/// ## Properties
///
/// Defined by [`HintProperties`].
#[function_component(Hint)]
pub fn hint(props: &HintProperties) -> Html {
    html! (
        <div
            id={props.id.clone()}
            class={classes!("pf-c-hint", props.class.clone())}
        >
            if let Some(actions) = &props.actions {
                <div class="pf-c-hint__actions">{ actions.clone() }</div>
            }
            if !props.children.is_empty() {
                { for props.children.iter() }
            }
        </div>
    )
}

/// Properties for [`HintTitle`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintTitleProperties {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// HintTitle component
///
/// ## Properties
///
/// Defined by [`HintTitleProperties`].
#[function_component(HintTitle)]
pub fn hint_title(props: &HintTitleProperties) -> Html {
    html! (
        <div
            id={props.id.clone()}
            class={classes!("pf-c-hint__title", props.class.clone())}
        >
            if !props.children.is_empty() {
                { for props.children.iter() }
            }
        </div>
    )
}

/// Properties for [`HintBody`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintBodyProperties {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// HintBody component
///
/// ## Properties
///
/// Defined by [`HintBodyProperties`].
#[function_component(HintBody)]
pub fn hint_body(props: &HintBodyProperties) -> Html {
    html! (
        <div
            id={props.id.clone()}
            class={classes!("pf-c-hint__body", props.class.clone())}
        >
            if !props.children.is_empty() {
                { for props.children.iter() }
            }
        </div>
    )
}

/// Properties for [`HintFooter`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintFooterProperties {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

/// HintFooter component
///
/// ## Properties
///
/// Defined by [`HintFooterProperties`].
#[function_component(HintFooter)]
pub fn hint_footer(props: &HintFooterProperties) -> Html {
    html! (
        <div
            id={props.id.clone()}
            class={classes!("pf-c-hint__footer", props.class.clone())}
        >
            if !props.children.is_empty() {
                { for props.children.iter() }
            }
        </div>
    )
}
