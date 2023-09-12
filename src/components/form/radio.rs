use crate::prelude::use_prop_id;
use std::mem::swap;
use yew::prelude::*;

/// Properties for [`Radio`].
#[derive(PartialEq, Properties)]
pub struct RadioProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub reversed: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub description: Children,

    #[prop_or_default]
    pub body: Children,
}

/// Radio button component
///
/// > A **radio** button is used to present the user with mutually exclusive choices. Always present radio buttons in groups of 2 or more.
///
/// See: <https://www.patternfly.org/components/forms/radio>
///
/// ## Properties
///
/// Defined by [`RadioProperties`].
#[function_component(Radio)]
pub fn radio(props: &RadioProperties) -> Html {
    let class = classes!("pf-v5-c-radio");

    let id = use_prop_id(props.id.clone());

    let mut input_class = classes!("pf-v5-c-radio__input");

    if props.children.is_empty() {
        input_class.extend(classes!("pf-m-standalone"));
    }

    let mut first = html!(
        <input
            class={input_class}
            type="radio"
            id={(*id).clone()}
            name={&props.name}
            checked={props.checked}
            disabled={props.disabled}
        />
    );

    let mut label_class = classes!("pf-v5-c-radio__label");
    if props.disabled {
        label_class.extend(classes!("pf-m-disabled"));
    }

    let mut second = html!(
        <>
            if !props.children.is_empty() {
                <label
                    class={label_class}
                    for={(*id).clone()}
                >
                    { for props.children.iter() }
                </label>
            }
        </>
    );

    if props.reversed {
        swap(&mut first, &mut second);
    }

    html!(
        <div {class}>
            {first} {second}

            if !props.description.is_empty() {
                <span class="pf-v5-c-radio__description">
                    { for props.description.iter() }
                </span>
            }

            if !props.body.is_empty() {
                <span class="pf-v5-c-radio__body">
                    { for props.body.iter() }
                </span>
            }
        </div>
    )
}
