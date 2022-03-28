#[derive(Clone, Debug)]
pub struct ValidationContext<T> {
    pub value: T,
    pub initial: bool,
}

impl<T> From<T> for ValidationContext<T> {
    fn from(value: T) -> Self {
        ValidationContext {
            value,
            initial: false,
        }
    }
}

#[derive(Clone)]
pub enum Validator<S, T> {
    None,
    Custom(std::rc::Rc<dyn Fn(ValidationContext<T>) -> S>),
}

impl<S, T> Validator<S, T> {
    pub fn is_custom(&self) -> bool {
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }

    pub fn run<C>(&self, ctx: C) -> Option<S>
    where
        C: Into<ValidationContext<T>>,
    {
        self.run_ctx(ctx.into())
    }

    pub fn run_ctx(&self, ctx: ValidationContext<T>) -> Option<S> {
        match self {
            Self::Custom(validator) => Some(validator(ctx)),
            _ => None,
        }
    }
}

impl<S, T> Default for Validator<S, T> {
    fn default() -> Self {
        Self::None
    }
}

/// Validators are equal if they are still None. Everything else is a change.
impl<S, T> PartialEq for Validator<S, T> {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Validator::None, Validator::None) => true,
            _ => false,
        }
    }
}

impl<F, S, T> From<F> for Validator<S, T>
where
    F: Fn(ValidationContext<T>) -> S + 'static,
{
    fn from(v: F) -> Self {
        Self::Custom(std::rc::Rc::new(v))
    }
}
