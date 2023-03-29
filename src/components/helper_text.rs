//! Helper text
//!
//! **NOTE:** While it looks similar to the [`Form`](crate::prelude::Form)'s helper text, it is
//! a different type.

use crate::{AsClasses, ExtendClasses, Icon};
use yew::prelude::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HelperTextState {
    #[default]
    Default,
    Intermediate,
    Warning,
    Success,
    Error,
}

impl AsClasses for HelperTextState {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Intermediate => classes.push(classes!("pf-m-indeterminate")),
            Self::Warning => classes.push(classes!("pf-m-warning")),
            Self::Success => classes.push(classes!("pf-m-success")),
            Self::Error => classes.push(classes!("pf-m-error")),
        }
    }
}

impl HelperTextState {
    pub fn icon(&self) -> Icon {
        match self {
            Self::Default => Icon::Minus,
            Self::Intermediate => Icon::Minus,
            Self::Warning => Icon::ExclamationTriangle,
            Self::Success => Icon::CheckCircle,
            Self::Error => Icon::ExclamationCircle,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HelperTextProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<HelperTextItem>,
}

/// Helper text
///
/// > **Helper text** is an on-screen field guideline that helps provide context regarding field inputs.
///
/// See: <https://www.patternfly.org/v4/components/helper-text>
///
/// ## Properties
///
/// Defined by [`HelperTextProperties`].
///
/// ## Children
///
/// This components contains one or more [`HelperTextItem`] components.
#[function_component(HelperText)]
pub fn helper_text(props: &HelperTextProperties) -> Html {
    html!(
        <div class="pf-c-helper-text">
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HelperTextItemProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub state: HelperTextState,
    /// Override default icon
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub dynamic: bool,
}

/// An item in a [`HelperText`] component.
///
/// ## Properties
///
/// Defined by [`HelperTextItemProperties`].
#[function_component(HelperTextItem)]
pub fn helper_text_item(props: &HelperTextItemProperties) -> Html {
    let mut class = classes!("pf-c-helper-text__item");

    class.extend_from(&props.state);

    if props.dynamic {
        class.push(classes!("pf-m-dynamic"));
    }

    let icon = props.icon.unwrap_or_else(|| props.state.icon());

    html!(
        <div {class}>
            <span class="pf-c-helper-text__item-icon">
                { icon }
            </span>
            <div class="pf-c-helper-text__item-text">
                { for props.children.iter() }
            </div>
        </div>
    )
}
