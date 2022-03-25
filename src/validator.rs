#[derive(Clone)]
pub enum Validator<S> {
    None,
    Custom(std::rc::Rc<dyn Fn(&str) -> S>),
}

impl<S> Validator<S> {
    pub fn is_custom(&self) -> bool {
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }

    pub fn run(&self, value: &str) -> Option<S> {
        match self {
            Self::Custom(validator) => Some(validator(value)),
            _ => None,
        }
    }
}

impl<S> Default for Validator<S> {
    fn default() -> Self {
        Self::None
    }
}

/// Validators are equal if they are still None. Everything else is a change.
impl<S> PartialEq for Validator<S> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Validator::None, Validator::None) => true,
            _ => false,
        }
    }
}

impl<F, S> From<F> for Validator<S>
where
    F: Fn(&str) -> S + 'static,
{
    fn from(v: F) -> Self {
        Self::Custom(std::rc::Rc::new(v))
    }
}
