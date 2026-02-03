//! Label
use crate::prelude::{AsClasses, Button, ButtonVariant, ExtendClasses, Icon};
use yew::prelude::*;

use strum_macros::{Display, EnumIter, EnumString};

#[derive(Copy, Clone, Default, Display, Debug, PartialEq, Eq, EnumIter, EnumString)]
pub enum Color {
    #[default]
    Grey,
    Blue,
    Green,
    Orange,
    Red,
    Purple,
    Teal,
    Yellow,
}

impl AsClasses for Color {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Color::Grey => {}
            Color::Blue => classes.push("pf-m-blue"),
            Color::Green => classes.push("pf-m-green"),
            Color::Orange => classes.push("pf-m-orange"),
            Color::Red => classes.push("pf-m-red"),
            Color::Purple => classes.push("pf-m-purple"),
            Color::Teal => classes.push("pf-m-teal"),
            Color::Yellow => classes.push("pf-m-yellow"),
        }
    }
}

/// Properties for [`Label`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LabelProperties {
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub color: Color,
    #[prop_or_default]
    pub outline: bool,
    #[prop_or_default]
    pub overflow: bool,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    /// Disables the label if it's clickable (if `onclick` is set) or removable (if `onclose` is set).
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub compact: bool,
}

/// Label component
///
/// > The **label** component allows users to add specific element captions for user clarity and convenience.
///
/// See: <https://www.patternfly.org/components/label/html>
///
/// ## Properties
///
/// Defined in [`LabelProperties`].
#[function_component(Label)]
pub fn label(props: &LabelProperties) -> Html {
    let mut classes = Classes::from("pf-v6-c-label");

    classes.extend_from(&props.color);

    if props.outline {
        classes.push("pf-m-outline");
    }

    if props.overflow {
        classes.push("pf-m-overflow");
    }

    if props.compact {
        classes.push("pf-m-compact");
    }

    if props.onclick.is_some() {
        classes.push("pf-m-clickable");
    }

    if props.disabled {
        classes.push("pf-m-disabled");
    }

    let content = |content: Html| {
        if let Some(onclick) = props.onclick.clone() {
            let onclick = Callback::from(move |_| onclick.emit(()));

            html! {<button class="pf-v6-c-label__content pf-m-clickable" {onclick} disabled={props.disabled}>{content}</button>}
        } else {
            html! {<span class="pf-v6-c-label__content">{content}</span>}
        }
    };

    html! (
        <span class={classes}>
            { content (
                html!(
                    <>
                        if let Some(icon) = &props.icon {
                            <span class="pf-v6-c-label__icon"> { icon.as_html() } </span>
                        }
                        <span class="pf-v6-c-label__text">
                            { &props.label }
                        </span>
                        if let Some(onclose) = &props.onclose {
                            <span class="pf-v6-c-label__actions">
                                <Button class={"pf-m-no-padding"} variant={ButtonVariant::Plain} icon={Icon::Times} onclick={onclose.reform(|_| {})} disabled={props.disabled}/>
                            </span>
                        }
                    </>
                )
            )}
        </span>
    )
}
