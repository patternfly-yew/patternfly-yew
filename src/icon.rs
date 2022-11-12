use crate::AsClasses;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::VNode;

pub enum State {
    None,
    Danger,
    Default,
    Info,
    Success,
    Warning,
    Disabled,
}

impl State {
    fn var(name: &str, weight: usize) -> String {
        format!("--pf-global--{}-color--{}", name, weight)
    }

    pub fn as_var(&self, weight: usize) -> Option<String> {
        match self {
            Self::None => None,
            Self::Danger => Some(Self::var("danger", weight)),
            Self::Default => Some(Self::var("default", weight)),
            Self::Info => Some(Self::var("info", weight)),
            Self::Success => Some(Self::var("success", weight)),
            Self::Warning => Some(Self::var("warning", weight)),
            Self::Disabled => Some(Self::var("disabled", weight)),
        }
    }
}

macro_rules! icons {
    ($($n:ident => $e:expr),* $(,)?) => {
        #[derive(Copy, Clone, Debug, PartialEq, Eq)]
        pub enum Icon {
            $($n),*
        }

        impl AsClasses for Icon {
            fn extend(&self, classes: &mut Classes) {
                match self {
                    $(Self::$n => classes.extend($e)),*
                }
            }
        }
    };
}

icons! {
    AngleDown => fas("fa-angle-down"),
    AngleLeft => fas("fa-angle-left"),
    AngleDoubleLeft => fas("fa-angle-double-left"),
    AngleRight => fas("fa-angle-right"),
    AngleDoubleRight => fas("fa-angle-double-right"),
    AngleUp => fas("fa-angle-up"),

    ArrowDown => fas("fa-arrow-down"),
    ArrowLeft => fas("fa-arrow-left"),
    ArrowRight => fas("fa-arrow-right"),
    ArrowUp => fas("fa-arrow-up"),

    Bell => fas("fa-bell"),

    CaretDown => fas("fa-caret-down"),
    CaretUp => fas("fa-caret-up"),
    Check => fas("fa-check"),
    CheckCircle => fas("fa-check-circle"),
    Copy => fas("fa-copy"),
    Cubes => fas("fa-cubes"),

    EllipsisH => fas("fa-ellipsis-h"),
    EllipsisV => fas("fa-ellipsis-v"),
    ExclamationCircle => fas("fa-exclamation-circle"),
    ExclamationTriangle => fas("fa-exclamation-triangle"),
    ExternalLinkAlt => fas("fa-external-link-alt"),

    InfoCircle => fas("fa-info-circle"),

    Minus => fas("fa-minus"),
    MinusCircleIcon => fas("fa-minus-circle"),

    Pause => fas("fa-pause"),
    Play => fas("fa-play"),
    PlusCircleIcon => fas("fa-plus-circle"),

    QuestionCircle => fas("fa-question-circle"),

    Times => fas("fa-times"),
    Th => fas("fa-th"),

    Help => pf("pf-icon-help"),
    Pending => pf("pf-icon-pending")
}

impl Icon {
    pub fn as_html(&self) -> Html {
        self.with_classes(Classes::new())
    }

    pub fn with_state(&self, state: State) -> Html {
        self.with_state_weight(state, 200)
    }

    pub fn with_state_weight(&self, state: State, weight: usize) -> Html {
        let style = state
            .as_var(weight)
            .map(|v| format!("color: var({});", v))
            .unwrap_or_default();

        html! {
            <span style={style}>
                { self.as_html() }
            </span>
        }
    }

    pub fn with_classes(&self, mut classes: Classes) -> Html {
        self.extend(&mut classes);

        html! {
            <i class={classes} aria-hidden="true"></i>
        }
    }
}

fn fas(name: &str) -> [&str; 2] {
    ["fas", name]
}

fn pf(name: &str) -> [&str; 2] {
    ["pficon", name]
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}

impl IntoPropValue<Html> for Icon {
    fn into_prop_value(self) -> Html {
        self.as_html()
    }
}

impl IntoPropValue<Option<Html>> for Icon {
    fn into_prop_value(self) -> Option<Html> {
        Some(self.as_html())
    }
}
