use crate::AsClasses;
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
    fn make_style(name: &str, weight: usize) -> String {
        format!("--pf-global--{}--color--{}", name, weight)
    }

    pub fn as_style(&self, weight: usize) -> String {
        match self {
            Self::None => "".into(),
            Self::Danger => Self::make_style("danger", weight),
            Self::Default => Self::make_style("default", weight),
            Self::Info => Self::make_style("info", weight),
            Self::Success => Self::make_style("success", weight),
            Self::Warning => Self::make_style("warning", weight),
            Self::Disabled => Self::make_style("disabled", weight),
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
            fn as_classes(&self) -> Classes {
                match self {
                    $(Self::$n => $e),*
                }
            }
        }
    };
}

icons! {
    AngleDown => fas("fa-angle-down"),
    AngleLeft => fas("fa-angle-left"),
    AngleRight => fas("fa-angle-right"),
    AngleUp => fas("fa-angle-up"),

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
        let icon_classes = self.as_classes();

        let style = state.as_style(200);

        html! {
            <i class=icon_classes style=style aria-hidden="true"></i>
        }
    }

    pub fn with_classes(&self, classes: Classes) -> Html {
        let icon_classes = self.as_classes();

        html! {
            <i class=(icon_classes, classes) aria-hidden="true"></i>
        }
    }
}

fn far(name: &str) -> Classes {
    let mut classes = Classes::new();
    classes.push("far");
    classes.push(name);
    classes
}

fn fas(name: &str) -> Classes {
    let mut classes = Classes::new();
    classes.push("fas");
    classes.push(name);
    classes
}

fn pf(name: &str) -> Classes {
    let mut classes = Classes::new();
    classes.push("pficon");
    classes.push(name);
    classes
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}
