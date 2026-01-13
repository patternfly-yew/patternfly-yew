use crate::prelude::AsClasses;
use yew::{Classes, classes};

/// Definition of visibility
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Visibility {
    #[default]
    Visible,
    Hidden,
}

impl AsClasses for Visibility {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Hidden => classes.push(classes!("pf-m-hidden")),
            Self::Visible => classes.push(classes!("pf-m-visible")),
        }
    }
}
