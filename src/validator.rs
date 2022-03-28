#[derive(Clone, Debug)]
pub struct ValidationContext {
    pub value: String,
    pub initial: bool,
}

impl From<String> for ValidationContext {
    fn from(value: String) -> Self {
        ValidationContext {
            value,
            initial: false,
        }
    }
}

#[derive(Clone)]
pub enum Validator<S> {
    None,
    Custom(std::rc::Rc<dyn Fn(ValidationContext) -> S>),
}

impl<S> Validator<S> {
    pub fn is_custom(&self) -> bool {
        match self {
            Self::Custom(_) => true,
            _ => false,
        }
    }

    pub fn run<C>(&self, ctx: C) -> Option<S>
    where
        C: Into<ValidationContext>,
    {
        self.run_ctx(ctx.into())
    }

    pub fn run_ctx(&self, ctx: ValidationContext) -> Option<S> {
        match self {
            Self::Custom(validator) => Some(validator(ctx)),
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
    F: Fn(ValidationContext) -> S + 'static,
{
    fn from(v: F) -> Self {
        Self::Custom(std::rc::Rc::new(v))
    }
}
