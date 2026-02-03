//! Hint

use yew::prelude::*;

/// Properties for [`Hint`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub actions: Option<Html>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

/// Hint component
///
/// > A **hint** is in-app messaging that provides a one-step reminder, explanation, or call to action for a page or modal.
///
/// See: <https://www.patternfly.org/components/hint>
///
/// ## Properties
///
/// Defined by [`HintProperties`].
#[function_component(Hint)]
pub fn hint(props: &HintProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-hint");

    html! (
        <div
            id={props.id.clone()}
            {class}
        >
            if let Some(actions) = &props.actions {
                <div class="pf-v6-c-hint__actions">{ actions.clone() }</div>
            }
            { props.children.clone() }
        </div>
    )
}

/// Properties for [`HintTitle`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintTitleProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

/// HintTitle component
///
/// ## Properties
///
/// Defined by [`HintTitleProperties`].
#[function_component(HintTitle)]
pub fn hint_title(props: &HintTitleProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-hint__title");

    html! (
        <div
            id={props.id.clone()}
            {class}
        >
            { props.children.clone() }
        </div>
    )
}

/// Properties for [`HintBody`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintBodyProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

/// HintBody component
///
/// ## Properties
///
/// Defined by [`HintBodyProperties`].
#[function_component(HintBody)]
pub fn hint_body(props: &HintBodyProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-hint__body");

    html! (
        <div
            id={props.id.clone()}
            {class}
        >
            { props.children.clone() }
        </div>
    )
}

/// Properties for [`HintFooter`]
#[derive(Clone, PartialEq, Properties)]
pub struct HintFooterProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub children: Html,
}

/// HintFooter component
///
/// ## Properties
///
/// Defined by [`HintFooterProperties`].
#[function_component(HintFooter)]
pub fn hint_footer(props: &HintFooterProperties) -> Html {
    let mut class = props.class.clone();
    class.push("pf-v6-c-hint__footer");

    html! (
        <div
            id={props.id.clone()}
            {class}
        >
            { props.children.clone() }
        </div>
    )
}
