//! Switch control
use crate::ouia;
use crate::prelude::{Icon, OuiaComponentType};
use crate::utils::{Ouia, OuiaSafe};
use web_tools::prelude::*;
use yew::prelude::*;

const OUIA: Ouia = ouia!("Switch");

/// Properties for [`Switch`]
#[derive(Clone, PartialEq, Properties)]
pub struct SwitchProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub checked: bool,

    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub label_off: Option<String>,
    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub onchange: Callback<bool>,

    #[prop_or_default]
    pub aria_label: String,

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

/// Switch component
///
/// > A **switch** toggles the state of a setting (between on and off). Switches and checkboxes can often be used interchangeably, but the switch provides a more explicit, visible representation on a setting.
///
/// See: <https://www.patternfly.org/components/switch>
///
/// ## Properties
///
/// Defined by [`SwitchProperties`].
#[function_component(Switch)]
pub fn switch(props: &SwitchProperties) -> Html {
    let checked = use_state(|| props.checked);

    use_effect_with(
        (props.checked, checked.setter()),
        |(checked, checked_setter)| {
            checked_setter.set(*checked);
        },
    );

    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let input_ref = use_node_ref();

    let onchange = use_callback(
        (input_ref.clone(), props.onchange.clone(), checked.setter()),
        |_evt, (input_ref, onchange, checked_setter)| {
            checked_setter.set(input_ref.checked());
            onchange.emit(input_ref.checked())
        },
    );

    let label = match (props.label.as_ref(), *checked) {
        (Some(label), true) => html!(
            <span class="pf-v6-c-switch__label">{label}</span>
        ),
        (Some(label), false) => {
            let label = props.label_off.as_ref().unwrap_or(label);
            html!(
                <span class="pf-v6-c-switch__label">{label}</span>
            )
        }
        (None, _) => html!(),
    };

    html! (
        <label
            class="pf-v6-c-switch"
            for={props.id.clone()}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <input
                ref={input_ref.clone()}
                class="pf-v6-c-switch__input"
                type="checkbox"
                id={props.id.clone()}
                aria-label={props.aria_label.clone()}
                checked={*checked}
                disabled={props.disabled}
                {onchange}
            />
            <span class="pf-v6-c-switch__toggle">
                if props.label.is_none() {
                    <span class="pf-v6-c-switch__toggle-icon">
                        { Icon::Check }
                    </span>
                }
            </span>
            {label}
        </label>
    )
}
