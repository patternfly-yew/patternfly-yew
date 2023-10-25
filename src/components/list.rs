//! List
use crate::icon::Icon;
use crate::prelude::{AsClasses, ExtendClasses};
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

impl IntoPropValue<Option<AttrValue>> for ListOrder {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(AttrValue::Static(match self {
            Self::Number => "1",
            Self::LowercaseLetter => "a",
            Self::UppercaseLetter => "A",
            Self::LowercaseRomanNumber => "i",
            Self::UppercaseRomanNumber => "I",
        }))
    }
}

/// Properties for [`List`]
#[derive(PartialEq, Properties)]
pub struct ListProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<ListItem>,
    #[prop_or_default]
    pub r#type: ListType,
    #[prop_or_default]
    pub icon_size: ListIconSize,
}

#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
pub enum ListIconSize {
    #[default]
    Default,
    Large,
}

impl AsClasses for ListIconSize {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Large => classes.extend(classes!("pf-m-icon-lg")),
        }
    }
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
/// Requires to use [`ListItem`] as children.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   html!(
///     <List>
///       <ListItem>{"Foo"}</ListItem>
///       <ListItem>{"Bar"}</ListItem>
///       <ListItem>{"Baz"}</ListItem>
///     </List>
///   )
/// }
/// ```
#[function_component(List)]
pub fn list(props: &ListProperties) -> Html {
    let mut classes = Classes::from("pf-v5-c-list");

    classes.extend_from(&props.r#type);
    classes.extend_from(&props.icon_size);

    let l = |items| match props.r#type {
        ListType::Basic | ListType::Inline | ListType::Plain | ListType::Bordered => {
            html! (<ul class={classes} role="list">{ items }</ul>)
        }
        ListType::Ordered(n) => {
            html! (<ol type={n} class={classes} role="list">{ items }</ol>)
        }
    };

    l(html! ({
         for props.children.clone()
    }))
}

#[derive(PartialEq, Properties)]
pub struct ListItemProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub icon: Option<Icon>,
}

#[function_component(ListItem)]
pub fn list_item(props: &ListItemProperties) -> Html {
    match props.icon {
        Some(icon) => {
            let class = classes!("pf-v5-c-list__item");
            html!(
                <li {class}>
                    <span class={classes!("pf-v5-c-list__item-icon")}>
                        { icon }
                    </span>
                    <span class={classes!("pf-v5-c-list__item-text")}>
                        { props.children.clone() }
                    </span>
                </li>
            )
        }
        None => html!( <li> { props.children.clone() } </li> ),
    }
}
