#![recursion_limit = "256"]
mod badge;
mod button;
mod card;
mod clipboard;
mod content;
mod form;
mod gallery;
mod logo;
mod nav;
mod page;
mod pagesection;
mod pagesidebar;
mod utils;

pub use badge::*;
pub use button::*;
pub use card::*;
pub use clipboard::*;
pub use content::*;
pub use form::*;
pub use gallery::*;
pub use logo::*;
pub use nav::*;
pub use page::*;
pub use pagesection::*;
pub use pagesidebar::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    Primary,
    Secondary,
    Link,
}

impl Variant {
    pub fn as_class(&self) -> &str {
        match self {
            Variant::Primary => "pf-m-primary",
            Variant::Secondary => "pf-m-secondary",
            Variant::Link => "pf-m-link",
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::Primary
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    PlusCircleIcon,
}

impl Icon {
    pub fn as_class(&self) -> &str {
        match self {
            Icon::PlusCircleIcon => "fas fa-plus-circle",
        }
    }
}
