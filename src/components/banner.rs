//! Page and application status banners.
use yew::prelude::*;

/// Color styling for [`Banner`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BannerColor {
    Red,
    OrangeRed,
    Orange,
    Yellow,
    Green,
    Teal,
    Blue,
    Purple,
}

impl BannerColor {
    fn class(self) -> &'static str {
        match self {
            Self::Red => "pf-m-red",
            Self::OrangeRed => "pf-m-orangered",
            Self::Orange => "pf-m-orange",
            Self::Yellow => "pf-m-yellow",
            Self::Green => "pf-m-green",
            Self::Teal => "pf-m-teal",
            Self::Blue => "pf-m-blue",
            Self::Purple => "pf-m-purple",
        }
    }
}

/// Status styling for [`Banner`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum BannerStatus {
    Success,
    Warning,
    Danger,
    Info,
    Custom,
}

impl BannerStatus {
    fn class(self) -> &'static str {
        match self {
            Self::Success => "pf-m-success",
            Self::Warning => "pf-m-warning",
            Self::Danger => "pf-m-danger",
            Self::Info => "pf-m-info",
            Self::Custom => "pf-m-custom",
        }
    }
}

/// Properties for [`Banner`].
#[derive(Clone, PartialEq, Properties)]
pub struct BannerProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
    /// Decorative color, overridden by `status` when supplied.
    #[prop_or_default]
    pub color: Option<BannerColor>,
    #[prop_or_default]
    pub status: Option<BannerStatus>,
    #[prop_or_default]
    pub sticky: bool,
    #[prop_or_default]
    pub pill: bool,
    /// Additional status text for screen readers. Omit for decorative banners.
    #[prop_or_default]
    pub screen_reader_text: AttrValue,
}

/// A banner for application-wide notices or environment labels.
///
/// Include an icon and screen-reader text when conveying status so that meaning
/// does not depend on color alone. Icons can be supplied through children.
///
/// See: <https://www.patternfly.org/components/banner>
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// let decorative = html!(<Banner color={BannerColor::Purple}>{"Development"}</Banner>);
/// let status = html!(
///     <Banner status={BannerStatus::Warning} screen_reader_text="Warning: " sticky=true>
///         { Icon::ExclamationTriangle }
///         { "Scheduled maintenance tonight" }
///     </Banner>
/// );
/// ```
#[function_component(Banner)]
pub fn banner(props: &BannerProperties) -> Html {
    render_banner(props)
}

fn render_banner(props: &BannerProperties) -> Html {
    let mut class = classes!("pf-v6-c-banner");
    class.push(
        props
            .status
            .map(BannerStatus::class)
            .or_else(|| props.color.map(BannerColor::class)),
    );
    if props.sticky {
        class.push("pf-m-sticky");
    }
    if props.pill {
        class.push("pf-m-pill");
    }
    class.extend(props.class.clone());
    html! {
        <div {class}>
            if !props.screen_reader_text.is_empty() {
                <span class="pf-v6-u-screen-reader">{props.screen_reader_text.clone()}</span>
            }
            {props.children.clone()}
        </div>
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn status_overrides_color_and_preserves_content() {
        let props = BannerProperties {
            color: Some(BannerColor::Red),
            status: Some(BannerStatus::Warning),
            sticky: true,
            pill: true,
            class: classes!("custom"),
            screen_reader_text: "Warning: ".into(),
            children: html!({ "Maintenance" }),
        };
        assert_eq!(
            render_banner(&props),
            html! {
                <div class={classes!("pf-v6-c-banner", "pf-m-warning", "pf-m-sticky", "pf-m-pill", "custom")}>
                    if true {
                    <span class="pf-v6-u-screen-reader">{"Warning: "}</span>
                }
                    {"Maintenance"}
                </div>
            }
        );
        let props = BannerProperties {
            status: None,
            sticky: false,
            pill: false,
            screen_reader_text: AttrValue::default(),
            class: Classes::new(),
            ..props
        };
        assert_eq!(
            render_banner(&props),
            html! {
                <div class={classes!("pf-v6-c-banner", "pf-m-red")}>if false { <span /> }{"Maintenance"}</div>
            }
        );
        let props = BannerProperties {
            color: None,
            ..props
        };
        assert_eq!(
            render_banner(&props),
            html! {
                <div class={classes!("pf-v6-c-banner")}>if false { <span /> }{"Maintenance"}</div>
            }
        );
    }
}
