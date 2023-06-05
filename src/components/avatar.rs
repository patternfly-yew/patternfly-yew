//! Avatar graphic
use yew::prelude::*;
use crate::Size;

/// Border style of the [`Avatar`] component
#[derive(Clone, Default, Debug, PartialEq)]
pub enum AvatarBorder {
    #[default]
    None,
    Dark,
    Light
}

impl From<AvatarBorder> for Classes {
    fn from(value: AvatarBorder) -> Self {
        classes!(
            match value {
                AvatarBorder::None => "",
                AvatarBorder::Dark => "pf-m-dark",
                AvatarBorder::Light => "pf-m-light",
            }
        )
    }
}

/// Size of the [`Border`] component
#[derive(Clone, Default, Debug, PartialEq)]
pub enum AvatarSize {
    #[default]
    None,
    Small,
    Medium,
    Large,
    XLarge,
}

impl From<AvatarSize> for Classes {
    fn from(value: AvatarSize) -> Self {
        classes!(
            match value {
                AvatarSize::None => Size::None,
                AvatarSize::Small => Size::Small,
                AvatarSize::Medium => Size::Medium,
                AvatarSize::Large => Size::Large,
                AvatarSize::XLarge => Size::XLarge,
            }
        )
    }
}


/// Properties for [`Avatar`]
#[derive(Clone, PartialEq, Properties)]
pub struct AvatarProperties {
    /// Required Attributes
    /// The image "alt" text.
    pub alt: AttrValue,

    #[prop_or_default]
    pub border: AvatarBorder,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub size: AvatarSize,
    /// The source of the image.
    #[prop_or_default]
    pub src: AttrValue,
}

/// Avatar component
///
/// > An **avatar** is a visual used to represent a user. It may contain an image or a placeholder graphic.
///
/// See: <https://pf5.patternfly.org/components/avatar>
///
/// ## Properties
///
/// Defined by [`AvatarProperties`].
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProperties) -> Html {
    html! {
        <img
            class={
                classes!(
                    "pf-v5-c-avatar",
                    props.border.clone(),
                    props.size.clone(),
                    props.class.clone()
                )
            }
            src={&props.src}
            alt={&props.alt}
        />
    }
}
