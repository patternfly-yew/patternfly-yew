#![recursion_limit = "256"]
mod badge;
mod button;
mod card;
mod clipboard;
mod content;
mod form;
mod gallery;
mod icon;
mod logo;
mod nav;
mod page;
mod pagesection;
mod pagesidebar;
mod table;
mod tooltip;
mod utils;

pub use badge::*;
pub use button::*;
pub use card::*;
pub use clipboard::*;
pub use content::*;
pub use form::*;
pub use gallery::*;
pub use icon::*;
pub use logo::*;
pub use nav::*;
pub use page::*;
pub use pagesection::*;
pub use pagesidebar::*;
pub use table::*;
pub use tooltip::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    None,
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
    Link,
    Control,
}

impl Variant {
    pub fn as_classes(&self) -> Vec<&str> {
        match self {
            Variant::None => vec![],
            Variant::Primary => vec!["pf-m-primary"],
            Variant::Secondary => vec!["pf-m-secondary"],
            Variant::Tertiary => vec!["pf-m-tertiary"],
            Variant::Warning => vec!["pf-m-warning"],
            Variant::Danger => vec!["pf-m-danger"],
            Variant::Link => vec!["pf-m-link"],
            Variant::Control => vec!["pf-m-control"],
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::None
    }
}
