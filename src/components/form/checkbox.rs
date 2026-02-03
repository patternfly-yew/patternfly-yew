use crate::ouia;
use crate::prelude::OuiaComponentType;
use crate::utils::OuiaSafe;
use crate::{core::OptionalHtml, hooks::id::use_prop_id, utils::Ouia};
use web_sys::HtmlInputElement;
use yew::html::IntoPropValue;
use yew::prelude::*;

const OUIA: Ouia = ouia!("Checkbox");

/// The state of a checkbox.
///
/// In addition to the obvious two states (checked and unchecked), a checkbox can also have an
/// "indeterminate" state.
///
/// This enum helps to work with this tri-state value. A boolean can easily converted into the
/// `CheckboxState`. When converting back to a boolean, only [`CheckboxState::Checked`] will turn
/// into `true`.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum CheckboxState {
    Checked,
    #[default]
    Unchecked,
    Indeterminate,
}

impl From<bool> for CheckboxState {
    fn from(b: bool) -> Self {
        match b {
            true => Self::Checked,
            false => Self::Unchecked,
        }
    }
}

impl From<CheckboxState> for bool {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Checked => true,
            CheckboxState::Indeterminate | CheckboxState::Unchecked => false,
        }
    }
}

impl From<CheckboxState> for Option<bool> {
    fn from(value: CheckboxState) -> Self {
        match value {
            CheckboxState::Checked => Some(true),
            CheckboxState::Unchecked => Some(false),
            CheckboxState::Indeterminate => None,
        }
    }
}

impl IntoPropValue<CheckboxState> for bool {
    fn into_prop_value(self) -> CheckboxState {
        self.into()
    }
}

/// Properties for [`Checkbox`].
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CheckboxProperties {
    /// Id of the checkbox
    #[prop_or_default]
    pub id: Option<String>,

    /// The name of the input field
    #[prop_or_default]
    pub name: Option<AttrValue>,

    /// Additional classes added to the checkbox.
    #[prop_or_default]
    pub class: Classes,

    /// Additional classes added to the radio input.
    #[prop_or_default]
    pub input_class: Classes,

    /// Flag to show if the checkbox selection is valid or invalid.
    #[prop_or_default]
    pub valid: bool,

    /// Flag to show if the checkbox is disabled.
    #[prop_or_default]
    pub disabled: bool,

    /// Flag to show if the checkbox is required.
    #[prop_or_default]
    pub required: bool,

    /// Flag to show if the checkbox is checked.
    #[prop_or_default]
    pub checked: CheckboxState,

    /// A callback for when the checkbox selection changes.
    #[prop_or_default]
    pub onchange: Callback<CheckboxState>,

    /// Label text of the checkbox.
    #[prop_or_default]
    pub label: OptionalHtml,

    /// Aria-label of the checkbox.
    #[prop_or_default]
    pub aria_label: AttrValue,

    /// Description text of the checkbox.
    #[prop_or_default]
    pub description: OptionalHtml,

    /// Body text of the checkbox.
    #[prop_or_default]
    pub body: Option<Html>,

    /// Sets the input wrapper component to render.
    #[prop_or(String::from("div"))]
    pub component: String,

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

/// Checkbox component
///
/// > A **checkbox** is used to select a single item or multiple items, typically to choose elements to perform an action or to reflect a binary setting.
///
/// See: <https://www.patternfly.org/components/forms/checkbox>
///
/// ## Properties
///
/// Defined by [`FormGroupProperties`].
#[function_component(Checkbox)]
pub fn checkbox(props: &CheckboxProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let id = use_prop_id(props.id.clone());
    let mut outer_class = classes!["pf-v6-c-check", props.class.clone()];

    if props.label.is_none() {
        outer_class.push("pf-m-standalone");
    }

    let node_ref = use_node_ref();

    {
        let node_ref = node_ref.clone();
        let checked = props.checked;
        use_effect(move || {
            if let Some(elem) = node_ref.cast::<HtmlInputElement>() {
                elem.set_indeterminate(checked == CheckboxState::Indeterminate)
            }
        });
    }

    let onchange = use_callback(
        (props.onchange.clone(), node_ref.clone()),
        |_: Event, (onchange, node_ref)| {
            let checked = node_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.checked().into())
                .unwrap_or_default();
            onchange.emit(checked);
        },
    );

    let label = if let Some(label) = &props.label.0 {
        let mut class = classes!["pf-v6-c-check__label"];
        if props.disabled {
            class.push("pf-m-disabled");
        }
        html! (
            <label {class} for={(*id).clone()}>
                {label.clone()}
                if props.required {
                    <span class="pf-v6-c-check__label-required" aria-hidden="true">{"*"}</span>
                }
            </label>
        )
    } else {
        html!()
    };

    html! (
        <@{props.component.clone()} class={outer_class}>
            <input
                class={classes!["pf-v6-c-check__input", props.input_class.clone()]}
                type="checkbox"
                {onchange}
                aria-invalid={props.valid.to_string()}
                aria-label={props.aria_label.clone()}
                disabled={props.disabled}
                required={props.required}
                id={(*id).clone()}
                ref={node_ref.clone()}
                checked={props.checked != CheckboxState::Unchecked}
                data-ouia-component-id={(*ouia_id).clone()}
                data-ouia-component-type={props.ouia_type}
                data-ouia-safe={props.ouia_safe}
            />
            {label}
            if let Some(description) = &props.description.0 {
                <span class="pf-v6-c-check__description">{description.clone()}</span>
            }
            if let Some(body) = &props.body {
                <span class="pf-v6-c-check__body">{body.clone()}</span>
            }
        </@>
    )
}
