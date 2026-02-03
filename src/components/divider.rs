//! General purpose dividers

use crate::prelude::{AsClasses, ExtendClasses, Inset, Visibility, WithBreakpoints};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerType {
    /// Use an `<hr>` element.
    #[default]
    Hr,
    /// Use an `<li>` element.
    Li,
    /// Use a `<div>` element.
    Div,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum DividerOrientation {
    #[default]
    Horizontal,
    Vertical,
}

impl AsClasses for DividerOrientation {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Horizontal => classes.push(classes!("pf-m-horizontal")),
            Self::Vertical => classes.push(classes!("pf-m-vertical")),
        }
    }
}

/// Properties for [`Divider`]
#[derive(Clone, Debug, PartialEq, Eq, Properties)]
pub struct DividerProperties {
    #[prop_or_default]
    pub r#type: DividerType,
    #[prop_or_default]
    pub orientation: WithBreakpoints<DividerOrientation>,
    #[prop_or_default]
    pub inset: WithBreakpoints<Inset>,
    #[prop_or_default]
    pub visibility: WithBreakpoints<Visibility>,
}

/// Divider component
///
/// > A **divider** is a horizontal or vertical line that is placed between screen elements to create visual divisions and content groupings.
///
/// See: <https://www.patternfly.org/components/divider>
///
/// ## Properties
///
/// Defined by [`DividerProperties`].
#[function_component(Divider)]
pub fn divider(props: &DividerProperties) -> Html {
    let mut class = classes!("pf-v6-c-divider");

    class.extend_from(&props.orientation);
    class.extend_from(&props.inset);
    class.extend_from(&props.visibility);

    match props.r#type {
        DividerType::Hr => html! (<hr {class} />),
        DividerType::Li => html! (<li {class} role="separator"></li>),
        DividerType::Div => html! (<div {class} role="separator"></div>),
    }
}

/// Specialized list divider component
///
/// This component is normally used as part of a list of items, like as part of the
/// [`AppLauncher`](crate::prelude::AppLauncher).
///
/// ## Properties
///
/// This component does not have properties.
#[function_component(ListDivider)]
pub fn list_divider() -> Html {
    html!(<Divider r#type={DividerType::Li} />)
}
