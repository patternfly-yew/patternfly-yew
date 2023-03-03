use yew::prelude::*;

/// Definition of an action.
#[derive(Clone, PartialEq, Default, Debug)]
pub struct Action {
    /// The label for the end user
    pub label: String,
    /// The callback to execute when the action is triggered
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

/// Allows converting something into an [`Action`] by providing a label.
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
