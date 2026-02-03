//! Form controls
mod area;
mod checkbox;
mod group;
mod input;
mod radio;
mod section;
mod select;
mod validation;

pub use area::*;
pub use checkbox::*;
pub use group::*;
pub use input::*;
pub use radio::*;
pub use section::*;
pub use select::*;
use std::collections::BTreeMap;
pub use validation::*;

use crate::prelude::{Alert, AlertType, AsClasses, Button, ExtendClasses, WithBreakpoints};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormHorizontal;

impl AsClasses for FormHorizontal {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push("pf-m-horizontal")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormAlert {
    pub r#type: AlertType,
    pub title: String,
    pub children: Html,
}

//
// Form
//

/// Properties for [`Form`]
#[derive(Clone, PartialEq, Properties)]
pub struct FormProperties {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub horizontal: WithBreakpoints<FormHorizontal>,

    #[prop_or_default]
    pub action: Option<String>,
    #[prop_or_default]
    pub method: Option<String>,

    #[prop_or_default]
    pub limit_width: bool,

    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub alert: Option<FormAlert>,

    /// Reports the overall validation state
    #[prop_or_default]
    pub onvalidated: Callback<InputState>,

    #[prop_or_default]
    pub validation_warning_title: Option<String>,
    #[prop_or_default]
    pub validation_error_title: Option<String>,

    #[prop_or_default]
    pub onsubmit: Callback<SubmitEvent>,
}

#[derive(Debug, Default, PartialEq, Eq)]
pub struct ValidationState {
    results: BTreeMap<String, ValidationResult>,
    state: InputState,
}

impl ValidationState {
    fn to_state(&self) -> InputState {
        let mut current = InputState::Default;
        for r in self.results.values() {
            if r.state > current {
                current = r.state;
            }
            if current == InputState::Error {
                break;
            }
        }
        current
    }

    fn push_state(&mut self, state: GroupValidationResult) -> bool {
        match state.1 {
            Some(result) => {
                self.results.insert(state.0, result);
            }
            None => {
                self.results.remove(&state.0);
            }
        }

        // update with diff

        let state = self.to_state();
        if self.state != state {
            self.state = state;
            true
        } else {
            false
        }
    }
}

#[derive(Clone, Default, PartialEq)]
pub struct ValidationFormContext {
    callback: Callback<GroupValidationResult>,
    state: InputState,
}

impl ValidationFormContext {
    pub fn new(callback: Callback<GroupValidationResult>, state: InputState) -> Self {
        Self { callback, state }
    }

    pub fn is_error(&self) -> bool {
        matches!(self.state, InputState::Error)
    }

    pub fn push_state(&self, state: GroupValidationResult) {
        self.callback.emit(state);
    }

    pub fn clear_state(&self, id: String) {
        self.callback.emit(GroupValidationResult(id, None));
    }
}

pub struct GroupValidationResult(pub String, pub Option<ValidationResult>);

/// The Form component.
///
/// > A **form** is a group of elements used to collect information from a user in a variety of contexts including in a modal, in a wizard, or on a page. Use cases for forms include tasks reliant on user-inputted information for completion like logging in, registering, configuring settings, or completing surveys.
///
/// See: <https://www.patternfly.org/components/form>
///
/// ## Properties
///
/// Defined by [`FormProperties`].
pub struct Form {
    validation: ValidationState,
}

#[doc(hidden)]
pub enum FormMsg {
    GroupValidationChanged(GroupValidationResult),
}

impl Component for Form {
    type Message = FormMsg;
    type Properties = FormProperties;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            validation: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMsg::GroupValidationChanged(state) => {
                let changed = self.validation.push_state(state);
                if changed {
                    ctx.props().onvalidated.emit(self.validation.state);
                }
                changed
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v6-c-form");

        classes.extend_from(&ctx.props().horizontal);

        if ctx.props().limit_width {
            classes.push("pf-m-limit-width");
        }

        let alert = &ctx.props().alert;
        let validation_alert = Self::make_alert(
            self.validation.state,
            (
                ctx.props()
                    .validation_warning_title
                    .as_deref()
                    .unwrap_or("The form contains fields with warnings."),
                &html!(),
            ),
            (
                ctx.props()
                    .validation_error_title
                    .as_deref()
                    .unwrap_or("The form contains fields with errors."),
                &html!(),
            ),
        );

        // reduce by severity

        let alert = match (alert, &validation_alert) {
            (None, None) => None,
            (Some(alert), None) | (None, Some(alert)) => Some(alert),
            (Some(props), Some(validation)) if validation.r#type > props.r#type => Some(validation),
            (Some(props), Some(_)) => Some(props),
        };

        let validation_context = ValidationFormContext::new(
            ctx.link().callback(FormMsg::GroupValidationChanged),
            self.validation.state,
        );

        html! (
            <ContextProvider<ValidationFormContext> context={validation_context} >
                <form
                    novalidate=true
                    class={classes}
                    id={ctx.props().id.clone()}
                    action={ctx.props().action.clone()}
                    method={ctx.props().method.clone()}
                    onsubmit={ctx.props().onsubmit.clone()}
                >

                    if let Some(alert) = alert {
                        <div class="pf-v6-c-form__alert">
                            <Alert
                                inline=true
                                r#type={alert.r#type}
                                title={alert.title.clone()}
                                >
                                { alert.children.clone() }
                            </Alert>
                        </div>
                    }

                    { ctx.props().children.clone() }

                </form>
            </ContextProvider<ValidationFormContext>>
        )
    }
}

impl Form {
    fn make_alert(
        state: InputState,
        warning: (&str, &Html),
        error: (&str, &Html),
    ) -> Option<FormAlert> {
        match state {
            InputState::Default | InputState::Success => None,
            InputState::Warning => Some(FormAlert {
                r#type: AlertType::Warning,
                title: warning.0.to_string(),
                children: warning.1.clone(),
            }),
            InputState::Error => Some(FormAlert {
                r#type: AlertType::Danger,
                title: error.0.to_string(),
                children: error.1.clone(),
            }),
        }
    }
}

//
// Action group
//

/// Properties for [`ActionGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProperties {
    pub children: ChildrenWithProps<Button>,
}

#[function_component(ActionGroup)]
pub fn action_group(props: &ActionGroupProperties) -> Html {
    html! {
        <div class="pf-v6-c-form__group pf-m-action">
            <div class="pf-v6-c-form__actions">
                { for props.children.iter() }
            </div>
        </div>
    }
}
