use yew::prelude::*;

#[derive(PartialEq)]
pub enum SpinnerSize {
    Sm,
    Md,
    Lg,
    Xl,
}

impl Default for SpinnerSize {
    fn default() -> Self {
        Self::Xl
    }
}

impl SpinnerSize {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Self::Sm => vec!["pf-m-sm"],
            Self::Md => vec!["pf-m-md"],
            Self::Lg => vec!["pf-m-lg"],
            Self::Xl => vec!["pf-m-xl"],
        }
    }
}

#[derive(PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub size: SpinnerSize,
    #[prop_or_default]
    pub diameter: String,
    #[prop_or(String::from("Loading..."))]
    pub aria_label: String,
}

pub struct Spinner;

impl Component for Spinner {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-spinner");
        classes.extend(ctx.props().size.as_classes());

        let diameter = ctx.props().diameter.clone();
        let style = if !diameter.is_empty() {
            format!("--pf-c-spinner--diameter: {};", diameter)
        } else {
            String::new()
        };

        html! {
            <svg
                class={classes}
                role="progressbar"
                viewBox="0 0 100 100"
                aria-label={ ctx.props().aria_label.clone() }
                { style }
            >
                <circle class="pf-c-spinner__path" cx="50" cy="50" r="45" fill="none" />
            </svg>
        }
    }
}
