use crate::InputState;

#[derive(Clone)]
pub enum Validator {
    None,
    Custom(std::rc::Rc<dyn Fn(&str) -> InputState>),
}

impl Validator {
    pub fn is_custom(&self) -> bool {
        match self {
            Validator::Custom(_) => true,
            _ => false,
        }
    }
}

impl Default for Validator {
    fn default() -> Self {
        Self::None
    }
}

/// Validators are equal if they are still None. Everything else is a change.
impl PartialEq for Validator {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Validator::None, Validator::None) => true,
            _ => false,
        }
    }
}

impl<F> From<F> for Validator
where
    F: Fn(&str) -> InputState + 'static,
{
    fn from(v: F) -> Self {
        Self::Custom(std::rc::Rc::new(v))
    }
}
