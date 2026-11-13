//! Avatar graphic
use crate::prelude::{Icon, Size};
use yew::prelude::*;

/// Border style of the [`Avatar`] component
#[derive(Clone, Default, Debug, PartialEq)]
pub enum AvatarBorder {
    #[default]
    None,
    Dark,
    Light,
}

impl From<AvatarBorder> for Classes {
    fn from(value: AvatarBorder) -> Self {
        classes!(match value {
            AvatarBorder::None => "",
            AvatarBorder::Dark => "pf-m-dark",
            AvatarBorder::Light => "pf-m-light",
        })
    }
}

/// Color of a colorful [`Avatar`]
///
/// Colorful avatars automatically include a border.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum AvatarColor {
    Red,
    OrangeRed,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Purple,
    Gray,
}

impl AvatarColor {
    /// All available colors.
    pub const ALL: [AvatarColor; 9] = [
        Self::Red,
        Self::OrangeRed,
        Self::Orange,
        Self::Yellow,
        Self::Green,
        Self::Teal,
        Self::Blue,
        Self::Purple,
        Self::Gray,
    ];
}

impl From<AvatarColor> for Classes {
    fn from(value: AvatarColor) -> Self {
        classes!(match value {
            AvatarColor::Red => "pf-m-red",
            AvatarColor::OrangeRed => "pf-m-orangered",
            AvatarColor::Orange => "pf-m-orange",
            AvatarColor::Yellow => "pf-m-yellow",
            AvatarColor::Green => "pf-m-green",
            AvatarColor::Teal => "pf-m-teal",
            AvatarColor::Blue => "pf-m-blue",
            AvatarColor::Purple => "pf-m-purple",
            AvatarColor::Gray => "pf-m-gray",
        })
    }
}

/// Size of the [`Avatar`] component
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
        classes!(match value {
            AvatarSize::None => Size::None,
            AvatarSize::Small => Size::Small,
            AvatarSize::Medium => Size::Medium,
            AvatarSize::Large => Size::Large,
            AvatarSize::XLarge => Size::XLarge,
        })
    }
}

/// Properties for [`Avatar`]
#[derive(Clone, PartialEq, Properties)]
pub struct AvatarProperties {
    /// Required Attributes
    /// The image "alt" text.
    ///
    /// For colorful avatars (with a `color` or `initials`), it is used as the `aria-label`.
    pub alt: AttrValue,

    /// Optional Attributes
    #[prop_or_default]
    pub border: AvatarBorder,
    #[prop_or_default]
    pub class: Classes,
    /// Render a colorful avatar using the given color.
    ///
    /// Without `initials`, a user icon is shown.
    #[prop_or_default]
    pub color: Option<AvatarColor>,
    /// Render a colorful avatar showing the given initials.
    ///
    /// It is recommended to pass only a single initial to ensure the text fits inside the avatar.
    #[prop_or_default]
    pub initials: Option<AttrValue>,
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
/// See: <https://www.patternfly.org/components/avatar>
///
/// ## Properties
///
/// Defined by [`AvatarProperties`].
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProperties) -> Html {
    if props.color.is_some() || props.initials.is_some() {
        let content = match &props.initials {
            Some(initials) => html!(
                <span class="pf-v6-c-avatar__initials" aria-hidden="true">{ initials }</span>
            ),
            None => Icon::User.as_html(),
        };

        return html! {
            <div
                class={classes!(
                    "pf-v6-c-avatar",
                    "pf-m-colorful",
                    props.color.map(Classes::from),
                    props.size.clone(),
                    props.class.clone()
                )}
                role="img"
                aria-label={&props.alt}
            >
                { content }
            </div>
        };
    }

    html! {
        <img
            class={classes!(
                    "pf-v6-c-avatar",
                    props.border.clone(),
                    props.size.clone(),
                    props.class.clone()
                )}
            src={&props.src}
            alt={&props.alt}
        />
    }
}
