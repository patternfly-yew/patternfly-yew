use crate::prelude::AsClasses;
use std::fmt::Formatter;
use yew::Classes;

/// Definition for inset
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum Inset {
    #[default]
    None,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

impl std::fmt::Display for Inset {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::None => f.write_str("pf-m-inset-none"),
            Self::Small => f.write_str("pf-m-inset-sm"),
            Self::Medium => f.write_str("pf-m-inset-md"),
            Self::Large => f.write_str("pf-m-inset-lg"),
            Self::XLarge => f.write_str("pf-m-inset-xl"),
            Self::XXLarge => f.write_str("pf-m-inset-2xl"),
        }
    }
}

impl AsClasses for Inset {
    fn extend_classes(&self, classes: &mut Classes) {
        // relies on the `Display` implementation above
        classes.push(self.to_string())
    }
}
