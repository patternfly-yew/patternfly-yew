use crate::AsClasses;
use yew::Classes;

/// Definition for sizes
#[derive(Clone, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Size {
    XSmall,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
    XXXLarge,
    XXXXLarge,
}

impl AsClasses for Size {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push(match self {
            Size::XSmall => "pf-m-xs",
            Size::Small => "pf-m-sm",
            Size::Medium => "pf-m-md",
            Size::Large => "pf-m-lg",
            Size::XLarge => "pf-m-xl",
            Size::XXLarge => "pf-m-2xl",
            Size::XXXLarge => "pf-m-3xl",
            Size::XXXXLarge => "pf-m-4xl",
        });
    }
}
