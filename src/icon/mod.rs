use crate::core::AsClasses;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::VNode;

mod generated;
mod state;

pub use generated::*;
pub use state::*;

impl Icon {
    pub fn as_html(&self) -> Html {
        self.with_classes(Classes::new())
    }

    pub fn with_state(&self, state: State) -> Html {
        self.with_state_weight(state, 200)
    }

    pub fn with_state_weight(&self, state: State, weight: usize) -> Html {
        let style = state.as_var(weight).map(|v| format!("color: var({});", v));

        html! (
            <span {style}>
                { self.as_html() }
            </span>
        )
    }

    pub fn with_classes(&self, mut classes: Classes) -> Html {
        self.extend_classes(&mut classes);

        html! (
            <i class={classes} aria-hidden="true"></i>
        )
    }
}

pub(crate) fn plain(name: &str) -> [&str; 1] {
    [name]
}

pub(crate) fn fas(name: &str) -> [&str; 2] {
    ["fas", name]
}

#[cfg(feature = "icons-far")]
pub(crate) fn far(name: &str) -> [&str; 2] {
    ["far", name]
}

#[cfg(feature = "icons-fab")]
pub(crate) fn fab(name: &str) -> [&str; 2] {
    ["fab", name]
}

pub(crate) fn pf(name: &str) -> [&str; 2] {
    ["pf-v5-pficon", name]
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
