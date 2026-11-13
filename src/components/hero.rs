//! Hero

use std::fmt::Write;
use yew::prelude::*;

/// Gradient of a [`Hero`]
///
/// Each stop accepts any valid CSS color. Stops which are not set keep the PatternFly default.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct HeroGradient {
    pub stop1: Option<AttrValue>,
    pub stop2: Option<AttrValue>,
    pub stop3: Option<AttrValue>,
}

impl HeroGradient {
    /// Create a new gradient from three CSS colors.
    pub fn new(
        stop1: impl Into<AttrValue>,
        stop2: impl Into<AttrValue>,
        stop3: impl Into<AttrValue>,
    ) -> Self {
        Self {
            stop1: Some(stop1.into()),
            stop2: Some(stop2.into()),
            stop3: Some(stop3.into()),
        }
    }

    fn write_style(&self, style: &mut String, theme: &str) {
        for (n, stop) in [(1, &self.stop1), (2, &self.stop2), (3, &self.stop3)] {
            if let Some(stop) = stop {
                let _ = write!(
                    style,
                    "--pf-v6-c-hero--gradient--stop-{n}--{theme}: {stop};"
                );
            }
        }
    }
}

/// Properties for [`Hero`]
#[derive(Clone, PartialEq, Properties)]
pub struct HeroProperties {
    /// Content of the hero.
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Additional classes added to the hero.
    #[prop_or_default]
    pub class: Classes,

    /// Additional inline style added to the hero.
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// Light theme background image path of the hero.
    #[prop_or_default]
    pub background_src_light: Option<AttrValue>,

    /// Dark theme background image path of the hero.
    #[prop_or_default]
    pub background_src_dark: Option<AttrValue>,

    /// Light theme gradient of the hero.
    #[prop_or_default]
    pub gradient_light: Option<HeroGradient>,

    /// Dark theme gradient of the hero.
    #[prop_or_default]
    pub gradient_dark: Option<HeroGradient>,

    /// Modifies the width of the hero body (any CSS length, e.g. `600px`).
    #[prop_or_default]
    pub body_width: Option<AttrValue>,

    /// Modifies the max-width of the hero body (any CSS length, e.g. `50%`).
    #[prop_or_default]
    pub body_max_width: Option<AttrValue>,

    /// Apply glass styling, when the glass theme (`pf-v6-theme-glass`) is active.
    ///
    /// **NOTE:** This is a beta feature of PatternFly.
    #[prop_or_default]
    pub glass: bool,
}

/// Hero component
///
/// > A **hero** is a compositional container used to introduce a page or feature, typically
/// > with a headline, a description, and call-to-action buttons.
///
/// See: <https://www.patternfly.org/components/hero>
///
/// **NOTE:** This component requires a version of the PatternFly CSS which includes the hero
/// component.
///
/// ## Properties
///
/// Defined by [`HeroProperties`].
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(
///         <Hero gradient_light={HeroGradient::new("#ece1ff", "#e0f0ff", "#fff")}>
///             <Title level={Level::H1}>{ "Welcome" }</Title>
///         </Hero>
///     )
/// }
/// ```
#[function_component(Hero)]
pub fn hero(props: &HeroProperties) -> Html {
    let class = classes!(
        "pf-v6-c-hero",
        props.glass.then_some("pf-m-glass"),
        props.class.clone()
    );

    let mut style = String::new();
    if let Some(src) = &props.background_src_light {
        let _ = write!(style, "--pf-v6-c-hero--BackgroundImage--light: url({src});");
    }
    if let Some(src) = &props.background_src_dark {
        let _ = write!(style, "--pf-v6-c-hero--BackgroundImage--dark: url({src});");
    }
    if let Some(gradient) = &props.gradient_light {
        gradient.write_style(&mut style, "light");
    }
    if let Some(gradient) = &props.gradient_dark {
        gradient.write_style(&mut style, "dark");
    }
    if let Some(width) = &props.body_width {
        let _ = write!(style, "--pf-v6-c-hero__body--Width: {width};");
    }
    if let Some(max_width) = &props.body_max_width {
        let _ = write!(style, "--pf-v6-c-hero__body--MaxWidth: {max_width};");
    }
    if let Some(extra) = &props.style {
        style.push_str(extra);
    }

    let style = (!style.is_empty()).then_some(style);

    html!(
        <div id={props.id.clone()} {class} {style}>
            <div class="pf-v6-c-hero__body">
                { props.children.clone() }
            </div>
        </div>
    )
}
