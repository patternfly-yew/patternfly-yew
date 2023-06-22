use crate::core::AsClasses;
use yew::{classes, Classes};

/// Definition for orientations
#[derive(Copy, Clone, PartialEq, Eq, Debug)]
pub enum Orientation {
    Left,
    Right,
    Top,
    Bottom,
}

impl AsClasses for Orientation {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Left => classes.extend(classes!("pf-m-left")),
            Self::Right => classes.extend(classes!("pf-m-right")),
            Self::Top => classes.extend(classes!("pf-m-top")),
            Self::Bottom => classes.extend(classes!("pf-m-bottom")),
        }
    }
}
