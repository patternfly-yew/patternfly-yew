use yew::prelude::*;

#[derive(Clone, PartialEq, Default, Debug)]
pub struct Action {
    pub label: String,
    pub callback: Callback<()>,
}

impl Action {
    pub fn new<S>(label: S, callback: Callback<()>) -> Self
    where
        S: ToString,
    {
        Self {
            label: label.to_string(),
            callback,
        }
    }
}

pub trait IntoAction {
    fn into_action<S: ToString>(self, label: S) -> Action;
}

impl IntoAction for Callback<()> {
    fn into_action<S>(self, label: S) -> Action
    where
        S: ToString,
    {
        Action::new(label, self)
    }
}
