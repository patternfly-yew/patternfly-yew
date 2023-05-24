//! Utilities

mod action;
mod context;
mod global_close;
mod html;
mod popper;
mod props;

pub use action::*;
pub use context::*;
pub use global_close::*;
pub use html::*;
pub use popper::*;
pub use props::*;

use std::fmt::{Debug, Display, Formatter};
use yew::{html::IntoPropValue, use_memo, AttrValue};

/// Create a random ID.
///
/// This is creating a random ID, using a v4 UUID.
pub fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    /// Get a new, random ID
    pub fn new() -> Self {
        Id(uuid::Uuid::new_v4())
    }
}

impl Default for Id {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl IntoPropValue<AttrValue> for Id {
    fn into_prop_value(self) -> AttrValue {
        self.to_string().into()
    }
}

impl IntoPropValue<Option<AttrValue>> for Id {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

/// Use a random ID
#[yew::hook]
pub fn use_random_id() -> yew::UseStateHandle<Id> {
    yew::use_state_eq(Id::new)
}

/// Use an ID from properties, or random if none was provided
///
/// This value will not change when re-rendering.
#[yew::hook]
pub fn use_prop_id<I>(id: I) -> std::rc::Rc<String>
where
    I: Into<Option<String>>,
{
    use_memo(|id| id.clone().unwrap_or_else(random_id), id.into())
}
