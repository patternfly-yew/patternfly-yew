mod generated;
mod state;

pub use generated::*;
pub use state::*;

use crate::{core::AsClasses, prelude::Styled};
use yew::html::ChildrenRenderer;
use yew::{html::IntoPropValue, prelude::*, virtual_dom::VNode};

impl Icon {
    pub fn as_html(&self) -> Html {
        self.with_classes(Classes::new())
    }

    /// Wrap an [`Icon`] with a CSS style
    pub fn with_style(&self, style: impl Into<AttrValue>) -> Styled<Icon> {
        Styled {
            content: *self,
            style: Some(style.into()),
        }
    }

    /// Wrap an [`Icon`] with a optional CSS style
    pub fn with_optional_style(&self, style: impl Into<Option<AttrValue>>) -> Styled<Icon> {
        Styled {
            content: *self,
            style: style.into(),
        }
    }

    pub fn with_state(&self, state: State) -> Html {
        self.with_state_weight(state, 200)
    }

    pub fn with_state_weight(&self, state: State, weight: usize) -> Html {
        let style = state
            .as_var(weight)
            .map(|v| format!("color: var({});", v).into());
        self.with_optional_style(style).into_html()
    }

    pub fn with_classes(&self, mut classes: Classes) -> Html {
        self.extend_classes(&mut classes);

        html! (
            <i class={classes} aria-hidden="true"></i>
        )
    }
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
    ["pf-v6-pficon", name]
}

impl From<Icon> for VNode {
    fn from(icon: Icon) -> Self {
        icon.as_html()
    }
}

impl IntoPropValue<Option<Html>> for Icon {
    fn into_prop_value(self) -> Option<Html> {
        Some(self.as_html())
    }
}

impl IntoPropValue<Html> for Icon {
    fn into_prop_value(self) -> Html {
        self.as_html()
    }
}

impl IntoPropValue<ChildrenRenderer<VNode>> for Icon {
    fn into_prop_value(self) -> ChildrenRenderer<VNode> {
        ChildrenRenderer::new(vec![self.into()])
    }
}
