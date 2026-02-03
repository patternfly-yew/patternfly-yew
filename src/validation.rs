//! Validation

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

#[derive(Clone, Default)]
pub enum Validator<T, S> {
    #[default]
    None,
    Custom(std::rc::Rc<dyn Fn(ValidationContext<T>) -> S>),
}

impl<T, S> Validator<T, S> {
    pub fn is_custom(&self) -> bool {
        matches!(self, Self::Custom(_))
    }

    /// Convert into the context and run
    pub fn run<C>(&self, ctx: C) -> Option<S>
    where
        C: Into<ValidationContext<T>>,
    {
        self.run_if(|| ctx.into())
    }

    /// Only convert when necessary, and run.
    pub fn run_if<F>(&self, f: F) -> Option<S>
    where
        F: FnOnce() -> ValidationContext<T>,
    {
        match self {
            Self::Custom(validator) => Some(validator(f())),
            _ => None,
        }
    }

    /// Run with the provided context.
    pub fn run_ctx(&self, ctx: ValidationContext<T>) -> Option<S> {
        match self {
            Self::Custom(validator) => Some(validator(ctx)),
            _ => None,
        }
    }
}

/// Validators are equal if they are still None. Everything else is a change.
impl<T, S> PartialEq for Validator<T, S> {
    fn eq(&self, other: &Self) -> bool {
        matches!((self, other), (Validator::None, Validator::None))
    }
}

impl<F, T, S> From<F> for Validator<T, S>
where
    F: Fn(ValidationContext<T>) -> S + 'static,
{
    fn from(v: F) -> Self {
        Self::Custom(std::rc::Rc::new(v))
    }
}

pub trait IntoValidator<T, S> {
    fn into_validator(self) -> Validator<T, S>;
}

impl<F, T, S> IntoValidator<T, S> for F
where
    F: Fn(ValidationContext<T>) -> S + 'static,
{
    fn into_validator(self) -> Validator<T, S> {
        Validator::Custom(std::rc::Rc::new(self))
    }
}
