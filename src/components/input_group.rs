//! Input group

use yew::prelude::*;

/// Properties for [`InputGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProperties {
    pub children: Children,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub aria_label: AttrValue,
}

/// Input group component
///
/// > An **input group** groups multiple related controls or inputs together so they appear as one control.
///
/// See: <https://www.patternfly.org/v4/components/input-group>
///
/// ## Properties
///
/// Defined in [`InputGroupProperties`].
///
/// ## Children
///
/// Input groups can have form elements as their children, and also make use of the
/// [`InputGroupText`] component to amend the input group with additional text or icons.
#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProperties) -> Html {
    let mut class = classes!("pf-v5-c-input-group");

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    html! (
        <div {class} aria-label={&props.aria_label} role="group">
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct InputGroupTextProperties {
    /// The element's ID
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub plain: bool,

    #[prop_or_default]
    pub children: Children,
}

/// Input group text, as child of [`InputGroup`]
#[function_component(InputGroupText)]
pub fn input_group_text(props: &InputGroupTextProperties) -> Html {
    let mut class = classes!("pf-v5-c-input-group__text");

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    html!(
        <span {class} id={&props.id}>
            { for props.children.iter() }
        </span>
    )
}
