use yew::prelude::*;
use yew::virtual_dom::VNode;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    AngleDown,
    AngleLeft,
    AngleRight,
    AngleUp,

    Copy,

    PlusCircleIcon,
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
        }
    }
}
fn fa(name: &str) -> Html {
    let mut classes = Classes::from("fas");
    classes.push(name);
    return html! {<i class=classes aria-hidden="true"></i>};
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}
