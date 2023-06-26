use crate::prelude::{AsClasses, ExtendClasses};
use yew::prelude::*;

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SkeletonFontSize {
    Sm,
    Md,
    Lg,
    Xl,
    Xxl,
    Xxxl,
    Xxxxl,
}

impl AsClasses for SkeletonFontSize {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Sm => classes.push(classes!("pf-m-text-sm")),
            Self::Md => classes.push(classes!("pf-m-text-sm")),
            Self::Lg => classes.push(classes!("pf-m-text-lg")),
            Self::Xl => classes.push(classes!("pf-m-text-xl")),
            Self::Xxl => classes.push(classes!("pf-m-text-2xl")),
            Self::Xxxl => classes.push(classes!("pf-m-text-3xl")),
            Self::Xxxxl => classes.push(classes!("pf-m-text-4xl")),
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq)]
pub enum SkeletonShape {
    Circle,
    Square,
}

impl AsClasses for SkeletonShape {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Circle => classes.push(classes!("pf-m-circle")),
            Self::Square => classes.push(classes!("pf-m-square")),
        }
    }
}

/// Properties for [`Skeleton`]
#[derive(Clone, PartialEq, Properties)]
pub struct SkeletonProperties {
    #[prop_or_default]
    pub width: Option<String>,

    #[prop_or_default]
    pub height: Option<String>,

    #[prop_or_default]
    pub font_size: Option<SkeletonFontSize>,

    #[prop_or_default]
    pub shape: Option<SkeletonShape>,

    #[prop_or_default]
    pub screenreader_text: String,

    /// additional styles
    #[prop_or_default]
    pub style: AttrValue,
}

/// The Skeleton component.
///
/// > A **skeleton** is a type of loading state that allows you to expose content incrementally. For content that may take a long time to load, use a progress bar in place of a skeleton.
///
/// See: <https://www.patternfly.org/v4/components/skeleton>
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// pub fn example() -> Html {
///   html!(
///     <Skeleton />
///   )
/// }
/// ```
#[function_component(Skeleton)]
pub fn skeleton(props: &SkeletonProperties) -> Html {
    let mut skeleton_classes = classes!("pf-v5-c-skeleton");
    match props.font_size {
        Some(val) => skeleton_classes.extend_from(&val),
        _ => {}
    }
    match props.shape {
        Some(val) => skeleton_classes.extend_from(&val),
        _ => {}
    }

    let mut skeleton_styles = vec![props.style.to_string()];
    match &props.width {
        Some(val) => {
            let style = format!("--pf-v5-c-skeleton--Width: {};", val);
            skeleton_styles.push(style.to_string());
        }
        _ => {}
    }
    match &props.height {
        Some(val) => {
            let style = format!("--pf-v5-c-skeleton--Height: {};", val);
            skeleton_styles.push(style.to_string());
        }
        _ => {}
    }

    html!(
        <div
            class={skeleton_classes}
            style={skeleton_styles.join(" ")}
        >
            <span class={"pf-v5-u-screen-reader"}>{&props.screenreader_text}</span>
        </div>
    )
}
