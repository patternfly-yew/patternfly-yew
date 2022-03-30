mod area;
mod group;
mod input;
mod section;
mod select;
mod validation;

pub use area::*;
pub use group::*;
pub use input::*;
pub use section::*;
pub use select::*;
use std::collections::BTreeMap;
pub use validation::*;

use crate::{Alert, Button, Type, WithBreakpoints};
use std::fmt::{Display, Formatter};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct FormHorizontal;

impl Display for FormHorizontal {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.write_str("pf-m-horizontal")
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct FormAlert {
    pub r#type: Type,
    pub title: String,
    pub children: Html,
}

//
// Form
//

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: Option<String>,

    #[prop_or_default]
    pub horizontal: WithBreakpoints<FormHorizontal>,

    #[prop_or_default]
    pub limit_width: bool,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub alert: Option<FormAlert>,

    /// Reports the overall validation state
    #[prop_or_default]
    pub onvalidated: Callback<InputState>,

    pub validation_warning_title: Option<String>,
    pub validation_error_title: Option<String>,
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

pub struct Form {
    validation: ValidationState,
}

pub enum Msg {
    GroupValidationChanged(GroupValidationResult),
}

impl Component for Form {
    type Message = Msg;
    type Properties = Props;

    fn create(_ctx: &Context<Self>) -> Self {
        Self {
            validation: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::GroupValidationChanged(state) => {
                let changed = self.validation.push_state(state);
                if changed {
                    ctx.props().onvalidated.emit(self.validation.state);
                }
                changed
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-form");

        classes.extend(ctx.props().horizontal.clone());

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
            ctx.link().callback(Msg::GroupValidationChanged),
            self.validation.state,
        );

        html! (
            <ContextProvider<ValidationFormContext> context={validation_context} >
                <form novalidate=true class={classes} id={ctx.props().id.clone()}>

                    if let Some(alert) = alert {
                        <div class="pf-c-form__alert">
                            <Alert
                                inline=true
                                r#type={alert.r#type}
                                title={alert.title.clone()}
                                >
                                { alert.children.clone() }
                            </Alert>
                        </div>
                    } else {
                        <div style="display: none;"></div>
                    }

                    { for ctx.props().children.iter() }

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
                r#type: Type::Warning,
                title: warning.0.to_string(),
                children: warning.1.clone(),
            }),
            InputState::Error => Some(FormAlert {
                r#type: Type::Danger,
                title: error.0.to_string(),
                children: error.1.clone(),
            }),
        }
    }
}

//
// Action group
//

#[derive(Clone, PartialEq, Properties)]
pub struct ActionGroupProps {
    pub children: ChildrenWithProps<Button>,
}

#[function_component(ActionGroup)]
pub fn action_group(props: &ActionGroupProps) -> Html {
    html! {
        <div class="pf-c-form__group pf-m-action">
            <div class="pf-c-form__group-control">
                <div class="pf-c-form__actions">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}

//
// Input group
//

#[derive(Clone, PartialEq, Properties)]
pub struct InputGroupProps {
    pub children: Children,
}

#[function_component(InputGroup)]
pub fn input_group(props: &InputGroupProps) -> Html {
    html! {
        <div class="pf-c-input-group">
            { for props.children.iter() }
        </div>
    }
}
