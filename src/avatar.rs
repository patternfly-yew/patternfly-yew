use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct AvatarProperties {
    /// The source of the image.
    #[prop_or_default]
    pub src: AttrValue,
    /// The image "alt" text.
    #[prop_or("Avatar image".into())]
    pub alt: AttrValue,
}

/// The avatar component.
///
/// > An **avatar** is a visual used to represent a user. It may contain an image or a placeholder graphic.
///
/// See: https://www.patternfly.org/v4/components/avatar
///
/// ## Properties
///
/// Defined by [`AvatarProperties`].
#[function_component(Avatar)]
pub fn avatar(props: &AvatarProperties) -> Html {
    html! {
        <img
            class="pf-c-avatar"
            src={&props.src}
            alt={&props.alt}
        />
    }
}
