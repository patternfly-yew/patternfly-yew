use crate::core::AsClasses;
use popper_rs::state::AttributesMap;
use yew::{Classes, classes};

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

impl Orientation {
    pub fn from_popper_data(attributes: &AttributesMap) -> Self {
        match attributes.get("data-popper-placement").map(|v| v.as_str()) {
            Some("top") => Self::Top,
            Some("left") => Self::Left,
            Some("right") => Self::Right,
            None | Some(_) => Self::Bottom,
        }
    }
}
