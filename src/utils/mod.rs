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

pub fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Id(uuid::Uuid);

impl Id {
    /// Get a new, random ID
    pub fn new() -> Id {
        Id(uuid::Uuid::new_v4())
    }
}

impl Display for Id {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[yew::hook]
pub fn use_id() -> yew::UseStateHandle<Id> {
    yew::use_state_eq(Id::new)
}
