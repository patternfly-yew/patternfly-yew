//! Button

use crate::prelude::{Icon, Spinner, SpinnerSize};
use web_sys::HtmlElement;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew::virtual_dom::AttrValue;

#[deprecated(
    since = "0.4.0",
    note = "The enum `Variant` in the Button component module was renamed to `ButtonVariant`"
)]
pub type Variant = ButtonVariant;

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ButtonVariant {
    #[default]
    None,
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
    DangerSecondary,
    Link,
    InlineLink,
    Control,
    Plain,
}

impl ButtonVariant {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Self::None => vec![],
            Self::Primary => vec!["pf-m-primary"],
            Self::Secondary => vec!["pf-m-secondary"],
            Self::Tertiary => vec!["pf-m-tertiary"],
            Self::Warning => vec!["pf-m-warning"],
            Self::Danger => vec!["pf-m-danger"],
            Self::DangerSecondary => vec!["pf-m-danger", "pf-m-secondary"],
            Self::Link => vec!["pf-m-link"],
            Self::InlineLink => vec!["pf-m-link", "pf-m-inline"],
            Self::Control => vec!["pf-m-control"],
            Self::Plain => vec!["pf-m-plain"],
        }
    }
}

#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Align {
    Start,
    End,
}

impl Default for Align {
    fn default() -> Self {
        Align::Start
    }
}

/// Button Type.
///
/// See: <https://developer.mozilla.org/en-US/docs/Web/HTML/Element/button#attr-type>
#[derive(Clone, Copy, Default, Eq, PartialEq, Debug)]
pub enum ButtonType {
    #[default]
    Button,
    Reset,
    Submit,
}

impl IntoPropValue<Option<AttrValue>> for ButtonType {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(AttrValue::Static(match self {
            Self::Submit => "submit",
            Self::Reset => "reset",
            Self::Button => "button",
        }))
    }
}

/// Properties for [`Button`]
#[derive(Clone, PartialEq, Properties)]
pub struct ButtonProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub variant: ButtonVariant,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub block: bool,

    #[prop_or_default]
    pub loading: bool,

    #[prop_or_default]
    pub aria_label: AttrValue,
    #[prop_or_default]
    pub aria_labelledby: AttrValue,
    #[prop_or_default]
    pub aria_haspopup: AttrValue,
    #[prop_or_default]
    pub aria_pressed: Option<AttrValue>,

    #[prop_or_default]
    pub r#type: ButtonType,

    #[prop_or_default]
    pub form: AttrValue,
    #[prop_or_default]
    pub formaction: AttrValue,

    #[prop_or_default]
    pub role: AttrValue,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub children: Children,
}

/// Button component
///
/// > A **button** is a box area or text that communicates and triggers user actions when clicked or selected. Buttons can be used to communicate and immediately trigger actions a user can take in an application, like submitting a form, canceling a process, or creating a new object. Buttons can also be used to take a user to a new location, like another page inside of a web application, or an external site such as help or documentation.
///
/// See: <https://www.patternfly.org/v4/components/button>
///
/// ## Properties
///
/// Defined by [`ButtonProperties`].
#[function_component(Button)]
pub fn button(props: &ButtonProperties) -> Html {
    let node_ref = use_node_ref();

    let mut classes: Classes = classes!(
        "pf-v5-c-button",
        props.class.clone(),
        props.variant.as_classes()
    );

    if props.expanded {
        classes.push("pf-m-expanded");
    }
    if props.block {
        classes.push("pf-m-block");
    }
    if props.loading {
        classes.push("pf-m-progress pf-m-in-progress")
    }

    let label = use_memo(
        |(label, icon, align)| {
            let mut classes = Classes::from("pf-v5-c-button__icon");

            match align {
                Align::Start => classes.push("pf-m-start"),
                Align::End => classes.push("pf-m-end"),
            }

            let icon = match icon {
                Some(i) => html! (
                    <span class={classes}>
                        { i.clone() }
                    </span>
                ),
                None => html!(),
            };

            let label = html!(label);

            match align {
                Align::Start => vec![icon, label],
                Align::End => vec![label, icon],
            }
        },
        (props.label.clone(), props.icon, props.align),
    );

    let onclick = {
        let onclick = props.onclick.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |evt| {
            // Blur (loose focus) on the button element, to remove the focus after clicking
            if let Some(node) = node_ref.cast::<HtmlElement>() {
                node.blur().ok();
            }
            onclick.emit(evt);
        })
    };

    html! (
         <button
             ref={node_ref}
             id={props.id.clone()}
             class={classes}
             style={props.style.clone()}
             disabled={props.disabled}
             type={props.r#type}
             {onclick}
             role={props.role.clone()}
             form={props.form.clone()}
             formaction={props.formaction.clone()}
             aria-label={props.aria_label.clone()}
             aria-pressed={props.aria_pressed.clone()}
         >
             if props.loading {
                 <span class="pf-v5-c-button__progress">
                     <Spinner size={SpinnerSize::Md} />
                 </span>
             }

             { (*label).clone() }
             { for props.children.iter() }

         </button>
    )
}
