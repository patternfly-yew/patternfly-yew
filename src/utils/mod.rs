mod action;
mod breakpoint;
mod classes;
mod context;
mod global_close;
mod orientation;
mod position;
mod size;
mod space;

pub use action::*;
pub use breakpoint::*;
pub use classes::*;
pub use context::*;
pub use global_close::*;
pub use orientation::*;
pub use position::*;
pub use size::*;
pub use space::*;
use std::fmt::{Debug, Display, Formatter};
use yew::use_memo;

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
    use_memo(|id| id.clone().unwrap_or_else(|| random_id()), id.into())
}
