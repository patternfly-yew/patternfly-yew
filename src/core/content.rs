use std::ops::{Deref, DerefMut};
use yew::html::IntoPropValue;
use yew::prelude::*;

/// A wrapper around [`Option<Html>`], making it easier to assign to components.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct OptionalHtml(pub Option<Html>);

impl Deref for OptionalHtml {
    type Target = Option<Html>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for OptionalHtml {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl IntoPropValue<Option<Html>> for OptionalHtml {
    fn into_prop_value(self) -> Option<Html> {
        self.0
    }
}

impl From<Html> for OptionalHtml {
    fn from(value: Html) -> Self {
        Self(Some(value))
    }
}

impl From<Option<Html>> for OptionalHtml {
    fn from(value: Option<Html>) -> Self {
        Self(value)
    }
}

impl From<&str> for OptionalHtml {
    fn from(value: &str) -> Self {
        Self(Some(Html::from(value)))
    }
}

impl From<String> for OptionalHtml {
    fn from(value: String) -> Self {
        Self(Some(Html::from(value)))
    }
}

impl IntoPropValue<OptionalHtml> for &str {
    fn into_prop_value(self) -> OptionalHtml {
        self.into()
    }
}

impl IntoPropValue<OptionalHtml> for String {
    fn into_prop_value(self) -> OptionalHtml {
        self.into()
    }
}

impl IntoPropValue<OptionalHtml> for Html {
    fn into_prop_value(self) -> OptionalHtml {
        self.into()
    }
}

impl IntoPropValue<OptionalHtml> for Option<Html> {
    fn into_prop_value(self) -> OptionalHtml {
        self.into()
    }
}

impl<COMP: BaseComponent> IntoPropValue<OptionalHtml> for yew::virtual_dom::VChild<COMP> {
    fn into_prop_value(self) -> OptionalHtml {
        let html: Html = self.into();
        html.into()
    }
}
