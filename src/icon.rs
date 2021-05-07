use crate::AsClasses;
use yew::prelude::*;
use yew::virtual_dom::VNode;

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
    AngleDown => fa("fa-angle-down"),
    AngleLeft => fa("fa-angle-left"),
    AngleRight => fa("fa-angle-right"),
    AngleUp => fa("fa-angle-up"),

    Bell => fa("fa-bell"),

    CaretDown => fa("fa-caret-down"),
    CaretUp => fa("fa-caret-up"),
    Check => fa("fa-check"),
    CheckCircle => fa("fa-check-circle"),
    Copy => fa("fa-copy"),
    Cubes => fa("fa-cubes"),

    EllipsisH => fa("fa-ellipsis-h"),
    EllipsisV => fa("fa-ellipsis-v"),
    ExclamationCircle => fa("fa-exclamation-circle"),
    ExclamationTriangle => fa("fa-exclamation-triangle"),
    ExternalLinkAlt => fa("fa-external-link-alt"),

    InfoCircle => fa("fa-info-circle"),

    Pause => fa("fa-pause"),
    Play => fa("fa-play"),
    PlusCircleIcon => fa("fa-plus-circle"),

    Times => fa("fa-times"),
    Th => fa("fa-th"),

    Help => pf("pf-icon-help"),
    Pending => pf("pf-icon-pending")
}

impl Icon {
    pub fn as_html(&self) -> Html {
        self.with_classes(Classes::new())
    }

    pub fn with_classes(&self, classes: Classes) -> Html {
        let icon_classes = self.as_classes();

        html! {
            <i class=(icon_classes, classes) aria-hidden="true"></i>
        }
    }
}

fn fa(name: &str) -> Classes {
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
