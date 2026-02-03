//! Helper text
//!
//! **NOTE:** While it looks similar to the [`Form`](crate::prelude::Form)'s helper text, it is
//! a different type.

use crate::prelude::{AsClasses, ExtendClasses, Icon};
use log::warn;
use std::fmt::{Display, Formatter};
use std::rc::Rc;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HelperTextComponent {
    #[default]
    Div,
    Ul,
}

impl Display for HelperTextComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Div => "div",
            Self::Ul => "ul",
        };

        f.write_str(value)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HelperTextItemVariant {
    #[default]
    Default,
    Intermediate,
    Warning,
    Success,
    Error,
}

impl AsClasses for HelperTextItemVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Intermediate => classes.push(classes!("pf-m-indeterminate")),
            Self::Warning => classes.push(classes!("pf-m-warning")),
            Self::Success => classes.push(classes!("pf-m-success")),
            Self::Error => classes.push(classes!("pf-m-error")),
        }
    }
}

impl HelperTextItemVariant {
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
    /// Adds an accessible label to the helper text when `component` is [`HelperTextComponent::Ul`].
    #[prop_or_default]
    pub aria_label: Option<AttrValue>,
    /// Content to be rendered inside the [`HelperText`] container. This must be a [`HelperTextItem`] component.
    #[prop_or_default]
    pub children: ChildrenWithProps<HelperTextItem>,
    /// Additional classes to be applied to the [`HelperText`] container.
    #[prop_or_default]
    pub class: Classes,
    /// Specify the html element of the [`HelperText`] container. Defaults to using a `div`.
    #[prop_or_default]
    pub component: HelperTextComponent,
    /// id for the helper text container. The value of this prop can be passed into a form
    /// component's `aria-describedby` property when you intend for all helper text items to be
    /// announced to assistive technologies.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Flag for indicating whether the helper text container is a live region. Use this prop when
    /// you expect or intend for any [`HelperTextItem`] within the container to be dynamically updated.
    #[prop_or_default]
    pub live_region: bool,

    // Not included in PF React, but is in the html spec.
    /// Hides the [`HelperText`]
    #[prop_or_default]
    pub hidden: bool,
}

/// HelperText component
///
/// > **HelperText** is an on-screen field guideline that helps provide context regarding field inputs.
///
/// See: <https://www.patternfly.org/components/helper-text>
///
/// ## Properties
///
/// Defined by [`HelperTextProperties`].
///
/// ## Children
///
/// This component may contain one or more [`HelperTextItem`] components.
///
/// ## Example
///
/// ```
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(
///         <HelperText>
///             <HelperTextItem>{"This is default helper text"}</HelperTextItem>
///         </HelperText>
///     )
/// }
/// ```
#[function_component(HelperText)]
pub fn helper_text(props: &HelperTextProperties) -> Html {
    let mut class = classes!("pf-v6-c-helper-text", props.class.clone());
    if props.hidden {
        class.push("pf-m-hidden")
    }
    let aria_live = props.live_region.then_some("polite");
    let component = props.component.to_string();
    let item_component = match props.component {
        HelperTextComponent::Div => HelperTextItemComponent::Div,
        HelperTextComponent::Ul => HelperTextItemComponent::Li,
    };
    let role = (props.component == HelperTextComponent::Ul).then_some("list");
    let _ = use_memo(
        (props.component, props.aria_label.clone()),
        // Use memo so that warn doesnt keep getting called.
        |(component, label)| {
            if component == &HelperTextComponent::Ul && label.is_none() {
                warn!(
                    "The aria_label property should be set on the HelperText component when the \
                    component attribute is set to HelperTextComponent::Ul"
                );
            }
        },
    );

    html!(
        <@{component}
            id={ &props.id }
            { class }
            aria-label={ &props.aria_label }
            aria-live={ aria_live }
            { role }
        >
            {
                for props.children.iter().map(|mut c|{
                    let props = Rc::make_mut(&mut c.props);
                    props.component = item_component;
                    c
                })
            }
        </@>
    )
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HelperTextItemComponent {
    #[default]
    Div,
    Li,
}

impl Display for HelperTextItemComponent {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let value = match self {
            Self::Div => "div",
            Self::Li => "li",
        };

        f.write_str(value)
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum HelperTextItemIcon {
    #[default]
    Default,
    Hidden,
    Visible,
    Custom(Icon),
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct HelperTextItemProperties {
    /// Content to be rendered inside the [`HelperTextItem`].
    #[prop_or_default]
    pub children: Html,
    /// Additional classes to be applied to the [`HelperTextItem`].
    #[prop_or_default]
    pub class: Classes,
    /// Set the type of html element to use. When [`HelperTextItem`] is used as a child of
    /// [`HelperText`] this property is set automatically.
    #[prop_or_default]
    pub component: HelperTextItemComponent,
    /// Flag to modifies a [`HelperTextItem`] to be dynamic. For use when the item changes state as
    /// the form field the text is associated with is updated.
    #[prop_or_default]
    pub dynamic: bool,
    /// Controls the icon prefixing the helper text. The default is to show an icon when the
    /// `dynamic` property is `true`
    #[prop_or_default]
    pub icon: HelperTextItemIcon,
    /// id for the [`HelperTextItem`]. The value of this property can be passed into a form
    /// component's `aria-describedby` property when you intend for only specific helper text items
    /// to be announced to assistive technologies.
    #[prop_or_default]
    pub id: Option<AttrValue>,
    /// Text that is only accessible to screen readers in order to announce the status of the
    /// [`HelperTextItem`]. This property is only used when the `dynamic` is `true`.
    #[prop_or_default]
    pub screen_reader_text: AttrValue,
    /// Variant styling of the helper text item
    #[prop_or_default]
    pub variant: HelperTextItemVariant,
}

/// An item in a [`HelperText`] component.
///
/// ## Properties
///
/// Defined by [`HelperTextItemProperties`].
///
/// ## Example
///
/// See [`HelperText`] for sample usage
#[function_component(HelperTextItem)]
pub fn helper_text_item(props: &HelperTextItemProperties) -> Html {
    let mut class = classes!("pf-v6-c-helper-text__item", props.class.clone());
    if props.dynamic {
        class.push(classes!("pf-m-dynamic"));
    }
    class.extend_from(&props.variant);
    let component = props.component.to_string();
    let icon = match (props.icon, &props.dynamic) {
        (HelperTextItemIcon::Default, false) | (HelperTextItemIcon::Hidden, ..) => None,
        (HelperTextItemIcon::Default, true) | (HelperTextItemIcon::Visible, ..) => {
            Some(props.variant.icon())
        }
        (HelperTextItemIcon::Custom(icon), ..) => Some(icon),
    };

    let item_icon = icon.map(|icon| {
        html!(
            <span class="pf-v6-c-helper-text__item-icon">
                { icon }
            </span>
        )
    });

    let screen_reader = use_memo(
        (props.screen_reader_text.clone(), props.dynamic),
        // Use memo so that warn doesn't keep getting called.
        |(text, _)| {
            if !text.is_empty() {
                if props.dynamic {
                    Some(html!(
                        <span class="pf-v6-u-screen-reader">
                            { ": " }{ text }{ ";" }
                        </span>
                    ))
                } else {
                    warn!(
                        "The screen_reader_text attribute was set but has not been used as the \
                    dynamic attribute was not set to true."
                    );
                    None
                }
            } else {
                None
            }
        },
    );

    html!(
        <@{component} id={ &props.id } { class }>
            { item_icon }
            <div class="pf-v6-c-helper-text__item-text">
                { props.children.clone() }
                { (*screen_reader).clone() }
            </div>
        </@>
    )
}
