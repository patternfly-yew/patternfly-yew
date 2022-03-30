use crate::{AsClasses, HelperText, Icon, ValidationContext};
use yew::{Callback, Classes};

#[derive(Copy, Clone, Debug, Eq, PartialEq, PartialOrd, Ord)]
pub enum InputState {
    Default,
    Success,
    Warning,
    Error,
}

impl Default for InputState {
    fn default() -> Self {
        Self::Default
    }
}

impl AsClasses for InputState {
    fn as_classes(&self) -> Classes {
        match self {
            Self::Default => Classes::default(),
            Self::Success => Classes::from_iter(["pf-m-success".to_string()]),
            Self::Warning => Classes::from_iter(["pf-m-warning".to_string()]),
            Self::Error => Classes::from_iter(["pf-m-error".to_string()]),
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
            InputState::Error => aria_invalid = true,
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

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ValidationResult {
    pub message: Option<String>,
    pub state: InputState,
}

impl From<ValidationResult> for Option<HelperText> {
    fn from(result: ValidationResult) -> Self {
        if matches!(result.state, InputState::Default) && result.message.is_none() {
            // default state and no message
            None
        } else {
            // non-default state or some message
            Some(HelperText {
                message: result.message.unwrap_or_default(),
                input_state: result.state,
                custom_icon: None,
                no_icon: match result.state {
                    InputState::Default => true,
                    _ => false,
                },
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

pub trait ValidatingComponent {
    type Value;
}

pub trait ValidatingComponentProperties<T> {
    fn set_onvalidate(&mut self, onvalidate: Callback<ValidationContext<T>>);
    fn set_input_state(&mut self, state: InputState);
}
