use crate::{AsClasses, ExtendClasses};
use std::fmt::Debug;
use yew::prelude::*;

/// Properties for [`TableColumn`]
#[derive(Debug, PartialEq, Eq, Clone, Properties)]
pub struct TableColumnProperties {
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub width: ColumnWidth,

    #[doc(hidden)]
    #[prop_or_default]
    pub(crate) first_tree_column: bool,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum ColumnWidth {
    #[default]
    Default,
    /// Percentage modifier
    ///
    /// From 10 to 90, rounded to the nearest ten. Values outside of the limit will be replaced
    /// with the limit.
    Percent(u16),
    /// Maximize width
    WidthMax,
    /// Minimize with, without triggering text wrapping
    FitContent,
}

fn round(p: u16) -> u16 {
    if p <= 10 {
        return 10;
    }
    if p >= 90 {
        return 90;
    }

    // round to the nearest ten
    ((p + 5) / 10) * 10
}

impl AsClasses for ColumnWidth {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Percent(p) => classes.push(classes!(format!("pf-m-width-{}", round(*p)))),
            Self::WidthMax => classes.push(classes!("pf-m-width-max")),
            Self::FitContent => classes.push(classes!("pf-m-fit-content")),
        }
    }
}

/// The Table Column component.
///
/// ## Properties
///
/// Define by [`TableColumnProperties`].
#[function_component(TableColumn)]
pub fn table_column(props: &TableColumnProperties) -> Html {
    let mut class = Classes::new();

    if props.first_tree_column {
        class.push(classes!("pf-c-table__tree-view-title-header-cell"));
    }

    if props.center {
        class.push(classes!("pf-m-center"));
    }

    class.extend_from(&props.width);

    match &props.label {
        None => html! (<td></td>),
        Some(label) => html! (
            <th {class} scope="col" role="columnheader">{ &label }</th>
        ),
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(0), 10);
        assert_eq!(round(10), 10);
        assert_eq!(round(50), 50);
        assert_eq!(round(54), 50);
        assert_eq!(round(55), 60);
        assert_eq!(round(56), 60);
        assert_eq!(round(100), 90);
        assert_eq!(round(100), 90);
        assert_eq!(round(200), 90);
    }
}
