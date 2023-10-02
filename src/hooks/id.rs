//! Hooks for handling IDs

use crate::prelude::{random_id, Id};
use yew::prelude::*;

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
    use_memo(id.into(), |id| id.clone().unwrap_or_else(random_id))
}
