use web_sys::HtmlInputElement;
use yew::prelude::*;

use crate::hooks::id::use_prop_id;

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

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct CheckProps {
    /// Id of the checkbox
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
    pub onchange: Callback<(Event, CheckboxState)>,

    /// Label text of the checkbox.
    #[prop_or_default]
    pub label: Option<Html>,

    /// Aria-label of the checkbox.
    #[prop_or_default]
    pub aria_label: AttrValue,

    /// Description text of the checkbox.
    #[prop_or_default]
    pub description: Option<Html>,

    /// Body text of the checkbox.
    #[prop_or_default]
    pub body: Option<Html>,

    /// Sets the input wraper component to render.
    #[prop_or(String::from("div"))]
    pub component: String,
}

#[function_component(Check)]
pub fn checkbox(props: &CheckProps) -> Html {
    let id = use_prop_id(props.id.clone());
    let mut outer_class = classes!["pf-v5-c-check", props.class.clone()];
    if props.label.is_none() {
        outer_class.push("pf-m-standalone")
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

    let onchange = {
        let onchange = props.onchange.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |e: Event| {
            let checked = node_ref
                .cast::<HtmlInputElement>()
                .map(|input| input.checked().into())
                .unwrap_or_default();
            onchange.emit((e, checked));
        })
    };
    let label = if let Some(label) = &props.label {
        let mut class = classes!["pf-v5-v-c-check__label"];
        if props.disabled {
            class.push("pf-m-disabled");
        }
        html! {
            <label {class} for={(*id).clone()}>
                {label.clone()}
                if props.required {
                    <span class="pf-v5-c-check__label-required" aria-hidden="true">{"*"}</span>
                }
            </label>
        }
    } else {
        html! {}
    };

    html! {
        <@{props.component.clone()} class={outer_class}>
            <input
                class={classes!["pf-v5-c-check__input", props.input_class.clone()]}
                type="checkbox"
                {onchange}
                aria-invalid={props.valid.to_string()}
                aria-label={props.aria_label.clone()}
                disabled={props.disabled}
                required={props.required}
                id={(*id).clone()}
                ref={node_ref.clone()}
                checked={props.checked != CheckboxState::Unchecked}
            />
            {label}
            if let Some(description) = &props.description {
                <span class="pf-v5-c-check__description">{description.clone()}</span>
            }
            if let Some(body) = &props.body {
                <span class="pf-v5-c-check__body">{body.clone()}</span>
            }
        </@>
    }
}
