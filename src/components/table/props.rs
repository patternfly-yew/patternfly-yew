use crate::prelude::AsClasses;
use yew::prelude::*;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq)]
pub enum TableMode {
    #[default]
    Default,
    Compact,
    CompactNoBorders,
    CompactExpandable,
    Expandable,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum TableGridMode {
    Medium,
    Large,
    XLarge,
    XXLarge,
}

impl AsClasses for TableGridMode {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Medium => classes.push(classes!("pf-m-grid-md")),
            Self::Large => classes.push(classes!("pf-m-grid-lg")),
            Self::XLarge => classes.push(classes!("pf-m-grid-xl")),
            Self::XXLarge => classes.push(classes!("pf-m-grid-2xl")),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TextModifier {
    Wrap,
    NoWrap,
    Truncate,
    BreakWord,
}

impl AsClasses for TextModifier {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Wrap => classes.extend(classes!("pf-m-wrap")),
            Self::NoWrap => classes.extend(classes!("pf-m-nowrap")),
            Self::Truncate => classes.extend(classes!("pf-m-truncate")),
            Self::BreakWord => classes.extend(classes!("pf-m-break-word")),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SpanModifiers {
    Truncate,
}

impl AsClasses for SpanModifiers {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Truncate => classes.push("pf-m-truncate"),
        }
    }
}
