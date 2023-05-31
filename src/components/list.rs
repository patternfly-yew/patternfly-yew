//! List
use crate::{AsClasses, ExtendClasses};
use yew::{html::IntoPropValue, prelude::*, virtual_dom::AttrValue};

#[derive(Copy, Clone, PartialEq, Eq)]
pub enum ListType {
    Basic,
    Inline,
    Ordered(ListOrder),
    Plain,
    Bordered,
}

impl AsClasses for ListType {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            ListType::Inline => {
                classes.push(classes!("pf-m-inline"));
            }
            ListType::Plain => {
                classes.push(classes!("pf-m-plain"));
            }
            ListType::Bordered => {
                classes.push(classes!("pf-m-plain", "pf-m-bordered"));
            }
            _ => {}
        }
    }
}

impl Default for ListType {
    fn default() -> Self {
        Self::Basic
    }
}

#[derive(Copy, Clone, Default, PartialEq, Eq)]
pub enum ListOrder {
    #[default]
    Number,
    LowercaseLetter,
    UppercaseLetter,
    LowercaseRomanNumber,
    UppercaseRomanNumber,
}

impl ToString for ListOrder {
    fn to_string(&self) -> String {
        match self {
            Self::Number => "1",
            Self::LowercaseLetter => "a",
            Self::UppercaseLetter => "A",
            Self::LowercaseRomanNumber => "i",
            Self::UppercaseRomanNumber => "I",
        }
        .into()
    }
}

impl IntoPropValue<Option<AttrValue>> for ListOrder {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.to_string().into())
    }
}

/// Properties for [`List`]
#[derive(Clone, PartialEq, Properties)]
pub struct ListProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub r#type: ListType,
}

/// List component
///
/// > A **list** component embeds a formatted list (bulleted or numbered list) into page content.
///
/// See: <https://www.patternfly.org/v4/components/list>
///
/// ## Properties
///
/// Defined by [`ListProperties`].
///
/// ## Children
///
/// Each children will be wrapped into an list item element.
#[function_component(List)]
pub fn list(props: &ListProperties) -> Html {
    let mut classes = Classes::from("pf-c-list");

    classes.extend_from(&props.r#type);

    let l = |items| match props.r#type {
        ListType::Basic | ListType::Inline | ListType::Plain | ListType::Bordered => {
            html! (<ul class={classes}>{ items }</ul>)
        }
        ListType::Ordered(n) => {
            html! (<ol r#type={n} class={classes}>{ items }</ol>)
        }
    };

    l(html! (
        {
         for props.children.iter()
            .map(|item|html!{<li>{item}</li>})
        }
    ))
}
