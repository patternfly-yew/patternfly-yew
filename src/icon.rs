use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    // Font Awesome
    AngleDown,
    AngleLeft,
    AngleRight,
    AngleUp,

    Bell,

    CheckCircle,
    Copy,
    Cubes,

    ExclamationCircle,
    ExclamationTriangle,
    ExternalLinkAltIcon,

    InfoCircle,

    Pause,
    Play,
    PlusCircleIcon,

    Times,

    // Patternfly
    Help,
    Pending,
}

impl Icon {
    pub fn as_html(&self) -> Html {
        self.with_classes(Classes::new())
    }

    pub fn with_classes(&self, classes: Classes) -> Html {
        let icon_classes = match self {
            Icon::AngleDown => fa("fa-angle-down"),
            Icon::AngleLeft => fa("fa-angle-left"),
            Icon::AngleRight => fa("fa-angle-right"),
            Icon::AngleUp => fa("fa-angle-up"),

            Icon::Bell => fa("fa-bell"),

            Icon::CheckCircle => fa("fa-check-circle"),
            Icon::Copy => fa("fa-copy"),
            Icon::Cubes => fa("fa-cubes"),

            Icon::ExclamationCircle => fa("fa-exclamation-circle"),
            Icon::ExclamationTriangle => fa("fa-exclamation-triangle"),
            Icon::ExternalLinkAltIcon => fa("fa-external-link-alt"),

            Icon::InfoCircle => fa("fa-info-circle"),

            Icon::Pause => fa("fa-pause"),
            Icon::Play => fa("fa-play"),
            Icon::PlusCircleIcon => fa("fa-plus-circle"),
            Icon::Times => fa("fa-times"),

            Icon::Help => pf("pf-icon-help"),
            Icon::Pending => pf("pf-icon-pending"),
        };

        html! {
            <i class=(icon_classes, classes) aria-hidden="true"></i>
        }
    }
}
fn fa(name: &str) -> Classes {
    let mut classes = Classes::from("fas");
    classes.push(name);
    classes
}

fn pf(name: &str) -> Classes {
    let mut classes = Classes::from("pficon");
    classes.push(name);
    classes
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}
