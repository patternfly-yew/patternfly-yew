use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Icon {
    Copy,
    PlusCircleIcon,
}

impl Icon {
    pub fn as_html(&self) -> Html {
        match self {
            Icon::Copy => fa("fa-copy"),
            Icon::PlusCircleIcon => fa("fa-plus-circle"),
        }
    }
}
fn fa(name: &str) -> Html {
    let mut classes = Classes::from("fas");
    classes.push(name);
    return html! {<i class=classes></i>};
}
