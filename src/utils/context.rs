use std::ops::Deref;
use yew::{BaseComponent, Callback, Component, Context, ContextHandle};

pub struct ContextWrapper<C>
where
    C: Clone + PartialEq + 'static,
{
    context: Option<C>,
    _handle: Option<ContextHandle<C>>,
}

impl<C> ContextWrapper<C>
where
    C: Clone + PartialEq + 'static,
{
    pub fn new(context: Option<(C, ContextHandle<C>)>) -> Self {
        let (context, handle) = match context {
            Some((context, handle)) => (Some(context), Some(handle)),
            None => (None, None),
        };
        Self {
            context,
            _handle: handle,
        }
    }

    pub fn with<COMP, F>(ctx: &Context<COMP>, cb: F) -> Self
    where
        COMP: BaseComponent,
        F: Fn(C) -> COMP::Message + 'static,
    {
        let cb = ctx.link().callback(cb);
        Self::new(ctx.link().context(cb))
    }

    pub fn set(&mut self, context: C) -> bool {
        let context = Some(context);
        if self.context != context {
            self.context = context;
            true
        } else {
            false
        }
    }
}

impl<C, COMP, F> From<(&Context<COMP>, F)> for ContextWrapper<C>
where
    COMP: Component,
    F: Fn(C) -> COMP::Message + 'static,
    C: PartialEq + Clone,
{
    fn from((ctx, f): (&Context<COMP>, F)) -> Self {
        let link = ctx.link().clone();

        Self::new(link.clone().context(Callback::from(move |v| {
            link.send_message(f(v));
        })))
    }
}

impl<C> Deref for ContextWrapper<C>
where
    C: PartialEq + Clone,
{
    type Target = Option<C>;

    fn deref(&self) -> &Self::Target {
        &self.context
    }
}
