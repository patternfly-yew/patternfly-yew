use crate::ouia;
use crate::prelude::OuiaComponentType;
use crate::utils::OuiaSafe;
use crate::{prelude::use_prop_id, utils::Ouia};
use std::mem::swap;
use yew::prelude::*;

const OUIA: Ouia = ouia!("Radio");

/// Properties for [`Radio`].
#[derive(PartialEq, Properties)]
pub struct RadioProperties {
    /// Additional classes added to the radio button.
    #[prop_or_default]
    pub class: Classes,

    /// Additional classes added to the underlying input tag.
    #[prop_or_default]
    pub input_class: Classes,

    #[prop_or_default]
    pub id: Option<String>,

    /// The input control name, used to group radio buttons together.
    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub value: Option<AttrValue>,

    /// The radio button label content.
    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub reversed: bool,

    #[prop_or_default]
    pub disabled: bool,

    /// A longer description text.
    #[prop_or_default]
    pub description: Option<Html>,

    /// Additional content aligned with the label.
    #[prop_or_default]
    pub body: Option<Html>,

    /// Event fired when the radio button is checked (but not when unchecked).
    #[prop_or_default]
    pub onchange: Callback<()>,

    /// Event fired when any part of the input is clicked. If you only want something to happen
    /// when the radio button itself is clicked then use `onchange`.
    #[prop_or_default]
    pub input_onclick: Option<Callback<MouseEvent>>,

    /// Creates a non-standalone input with a label, even if there are no children to this radio.
    #[prop_or_default]
    pub force_label: bool,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
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
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let class = classes!("pf-v6-c-radio", props.class.clone());

    let id = use_prop_id(props.id.clone());

    let mut input_class = classes!(props.input_class.clone(), "pf-v6-c-radio__input");

    if props.children.is_empty() && !props.force_label {
        input_class.extend(classes!("pf-m-standalone"));
    }

    let onchange = use_callback(props.onchange.clone(), |_, onchange| {
        onchange.emit(());
    });

    let mut first = html!(
        <input
            class={input_class}
            type="radio"
            id={(*id).clone()}
            name={&props.name}
            checked={props.checked}
            disabled={props.disabled}
            value={&props.value}
            {onchange}
            onclick={props.input_onclick.clone()}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        />
    );

    let mut label_class = classes!("pf-v6-c-radio__label");
    if props.disabled {
        label_class.extend(classes!("pf-m-disabled"));
    }

    let mut second = html!(
        <>
            if !props.children.is_empty() || props.force_label {
                <label
                    class={label_class}
                    for={(*id).clone()}
                >
                    { props.children.clone() }
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

            if let Some(description) = &props.description {
                <span class="pf-v6-c-radio__description">
                    { description.clone() }
                </span>
            }

            if  let Some(body) = &props.body {
                <span class="pf-v6-c-radio__body">
                    { body.clone() }
                </span>
            }
        </div>
    )
}
