use crate::ExtendClasses;
use yew::prelude::*;

#[derive(PartialEq)]
pub enum SpinnerSize {
    None,
    Sm,
    Md,
    Lg,
    Xl,
    Custom(String),
}

impl Default for SpinnerSize {
    fn default() -> Self {
        Self::None
    }
}

impl SpinnerSize {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Self::None => Vec::new(),
            Self::Sm => vec!["pf-m-sm"],
            Self::Md => vec!["pf-m-md"],
            Self::Lg => vec!["pf-m-lg"],
            Self::Xl => vec!["pf-m-xl"],
            Self::Custom(_) => Vec::new(),
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct SpinnerProperties {
    #[prop_or_default]
    pub size: SpinnerSize,
    #[prop_or(String::from("Loading..."))]
    pub aria_label: String,
}

/// Spinner component
///
/// > A **spinner** is used to indicate to users that an action is in progress. For actions that may take a long time, use a progress bar instead.
///
/// See: <https://www.patternfly.org/v4/components/spinner>
///
/// ## Properties
///
/// Defined by [`SpinnerProperties`].
pub struct Spinner;

impl Component for Spinner {
    type Message = ();
    type Properties = SpinnerProperties;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-spinner");

        classes.extend_from(&ctx.props().size);

        let style = if let SpinnerSize::Custom(diameter) = &ctx.props().size {
            format!("--pf-c-spinner--diameter: {};", diameter)
        } else {
            String::new()
        };

        html! (
            <svg
                class={classes}
                role="progressbar"
                viewBox="0 0 100 100"
                aria-label={ ctx.props().aria_label.clone() }
                { style }
            >
                <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
            </svg>
        )
    }
}
