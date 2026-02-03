//! Background image
use std::borrow::Cow;
use yew::html::IntoPropValue;
use yew::prelude::*;

/// Properties for [`Background`]
#[derive(Clone, PartialEq, Properties)]
pub struct BackgroundProperties {
    /// The main element's ID.
    #[prop_or_default]
    pub id: Option<String>,

    /// The styling of the background.
    ///
    /// By default, this will be the patternfly logo loaded from the static PatternFly assets.
    /// However, this can be overridden by either using a simple URL for an image, or a full set
    /// of CSS styling.
    #[prop_or_default]
    pub style: BackgroundStyle,

    /// Additional CSS styling which will be appended to the base style.
    ///
    /// **NOTE:** Using this in combination with [`BackgroundStyle::Style`] will simple append
    /// two strings as style information.
    #[prop_or_default]
    pub additional_style: Option<String>,
}

#[derive(Clone, Default, Debug, PartialEq, Eq)]
pub enum BackgroundStyle {
    #[default]
    Default,
    Image(Cow<'static, str>),
    Style(Cow<'static, str>),
}

impl IntoPropValue<BackgroundStyle> for String {
    fn into_prop_value(self) -> BackgroundStyle {
        BackgroundStyle::Image(self.into())
    }
}

impl IntoPropValue<BackgroundStyle> for &'static str {
    fn into_prop_value(self) -> BackgroundStyle {
        BackgroundStyle::Image(self.into())
    }
}

impl IntoPropValue<BackgroundStyle> for Cow<'static, str> {
    fn into_prop_value(self) -> BackgroundStyle {
        BackgroundStyle::Image(self)
    }
}

/// Background image component
///
/// > A **background image** allows you to place an image in the background of your page or area of a page.
///
/// See: <https://www.patternfly.org/components/background-image>
///
/// ## Properties
///
/// Defined by [`BackgroundProperties`].
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   html!(
///     <>
///       <Background style="assets/images/pfbg-icon.svg"/>
///       <p>{"Content on the background"}</p>
///     </>
///   )
/// }
/// ```
#[function_component(Background)]
pub fn background(props: &BackgroundProperties) -> Html {
    let mut style = match &props.style {
        BackgroundStyle::Default => {
            "--pf-v6-c-background-image--BackgroundImage: url(assets/images/pfbg-icon.svg);"
                .to_string()
        }
        BackgroundStyle::Image(url) => {
            format!("--pf-v6-c-background-image--BackgroundImage: url({url});")
        }
        BackgroundStyle::Style(style) => style.to_string(),
    };

    if let Some(additional) = &props.additional_style {
        style.push_str(additional);
    }

    html!(
        <div
            id={props.id.clone()}
            class="pf-v6-c-background-image"
            {style}
        ></div>
    )
}
