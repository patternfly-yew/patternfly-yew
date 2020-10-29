use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    // Font Awesome
    AngleDown,
    AngleLeft,
    AngleRight,
    AngleUp,

    Copy,

    PlusCircleIcon,

    Times,

    // Patternfly
    Help,
}

impl Icon {
    pub fn as_html(&self) -> Html {
        match self {
            Icon::AngleDown => fa("fa-angle-down"),
            Icon::AngleLeft => fa("fa-angle-left"),
            Icon::AngleRight => fa("fa-angle-right"),
            Icon::AngleUp => fa("fa-angle-up"),

            Icon::Copy => fa("fa-copy"),

            Icon::PlusCircleIcon => fa("fa-plus-circle"),
            Icon::Times => fa("fa-times"),

            Icon::Help => pf("pf-icon-help"),
        }
    }
}
fn fa(name: &str) -> Html {
    let mut classes = Classes::from("fas");
    classes.push(name);
    return html! {<i class=classes aria-hidden="true"></i>};
}

fn pf(name: &str) -> Html {
    let mut classes = Classes::from("pficon");
    classes.push(name);
    return html! {<i class=classes aria-hidden="true"></i>};
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}
