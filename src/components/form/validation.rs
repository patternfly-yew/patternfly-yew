use crate::prelude::{AsClasses, FormHelperText, Icon, ValidationContext};
use yew::{Callback, Classes};

/// State of an input from validation
#[derive(Copy, Clone, Debug, Default, Eq, PartialEq, PartialOrd, Ord)]
pub enum InputState {
    #[default]
    Default,
    Success,
    Warning,
    Error,
}

impl AsClasses for InputState {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Success => classes.push("pf-m-success"),
            Self::Warning => classes.push("pf-m-warning"),
            Self::Error => classes.push("pf-m-error"),
        }
    }
}

impl InputState {
    pub fn convert(&self, mut classes: Classes) -> (Classes, bool) {
        let mut aria_invalid = false;
        match self {
            InputState::Default => {}
            InputState::Success => classes.push("pf-m-success"),
            InputState::Warning => classes.push("pf-m-warning"),
            InputState::Error => {
                classes.push("pf-m-error");
                aria_invalid = true;
            }
        };
        (classes, aria_invalid)
    }

    pub fn icon(&self) -> Icon {
        match self {
            Self::Default => Icon::Minus,
            Self::Success => Icon::CheckCircle,
            Self::Warning => Icon::ExclamationTriangle,
            Self::Error => Icon::ExclamationCircle,
        }
    }
}

/// Result of the validation
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationResult {
    pub message: Option<String>,
    pub state: InputState,
}

impl From<ValidationResult> for Option<FormHelperText> {
    fn from(result: ValidationResult) -> Self {
        if matches!(result.state, InputState::Default) || result.message.is_none() {
            // default state and no message
            None
        } else {
            // non-default state or some message
            Some(FormHelperText {
                message: result.message.unwrap_or_default(),
                input_state: result.state,
                custom_icon: None,
                no_icon: matches!(result.state, InputState::Default),
                is_dynamic: true,
            })
        }
    }
}

impl Default for ValidationResult {
    fn default() -> Self {
        ValidationResult::ok()
    }
}

impl ValidationResult {
    pub fn ok() -> Self {
        Self {
            message: None,
            state: Default::default(),
        }
    }

    pub fn new<S: Into<String>>(state: InputState, message: S) -> Self {
        Self {
            state,
            message: Some(message.into()),
        }
    }

    pub fn help<S: Into<String>>(message: S) -> Self {
        Self::new(InputState::Default, message)
    }

    pub fn error<S: Into<String>>(message: S) -> Self {
        Self::new(InputState::Error, message)
    }

    pub fn warning<S: Into<String>>(message: S) -> Self {
        Self::new(InputState::Warning, message)
    }
}

/// A component supporting validation.
pub trait ValidatingComponent {
    type Value;
}

/// A trait which components supporting validatio must implement.
pub trait ValidatingComponentProperties<T> {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<T>>);
    fn set_input_state(&mut self, state: InputState);
}
