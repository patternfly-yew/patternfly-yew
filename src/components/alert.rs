//! Alert popup

use crate::{Action, Button, ButtonVariant, Icon};

use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum Type {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

impl Type {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            Type::Default => vec![],
            Type::Info => vec!["pf-m-info"],
            Type::Success => vec!["pf-m-success"],
            Type::Warning => vec!["pf-m-warning"],
            Type::Danger => vec!["pf-m-danger"],
        }
    }

    pub fn aria_label(&self) -> &'static str {
        match self {
            Type::Default => "Default alert",
            Type::Info => "Information alert",
            Type::Success => "Success alert",
            Type::Warning => "Warning alert",
            Type::Danger => "Danger alert",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            Type::Default => Icon::Bell,
            Type::Info => Icon::InfoCircle,
            Type::Success => Icon::CheckCircle,
            Type::Warning => Icon::ExclamationTriangle,
            Type::Danger => Icon::ExclamationCircle,
        }
    }
}

/// Properties for [`Alert`]
#[derive(Clone, PartialEq, Properties)]
pub struct AlertProperties {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub r#type: Type,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub truncate: bool,
    #[prop_or_default]
    pub actions: Vec<Action>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}

/// Alert component
///
/// > An **alert** is a notification that provides brief information to the user without blocking their workflow.
///
/// See: <https://www.patternfly.org/v4/components/alert>
///
/// ## Properties
///
/// Defined by [`AlertProperties`].
#[function_component(Alert)]
pub fn alert(props: &AlertProperties) -> Html {
    let mut classes = classes!("pf-c-alert");

    classes.extend(props.r#type.as_classes());

    if props.inline {
        classes.push("pf-m-inline");
    }

    if props.truncate {
        classes.push("pf-m-truncate");
    }

    let t = props.r#type;

    let actions = if props.actions.is_empty() {
        html!()
    } else {
        html! (
            <div class="pf-c-alert__action-group">
                {for props.actions.iter().map(|action|{
                    html!{
                        <Button
                            variant={ButtonVariant::InlineLink}
                            label={action.label.clone()}
                            onclick={action.callback.reform(|_|())}
                        />
                    }
                })}
            </div>
        )
    };

    html! (
        <div id={props.id.clone()} class={classes} aria_label={t.aria_label()}>
            <div class="pf-c-alert__icon">{ t.icon() }</div>
            <div class="pf-c-alert__title">
                <strong>
                    <span class="pf-screen-reader">{ t.aria_label() }{":"}</span>
                    { &props.title }
                </strong>
            </div>


            if let Some(onclose) = props.onclose.as_ref() {
                <div class="pf-c-alert__action">
                    <Button variant={ButtonVariant::Plain} icon={Icon::Times} onclick={onclose.clone().reform(|_|())} />
                </div>
            }


            if !props.children.is_empty() {
                <div class="pf-c-alert__description">
                    { for props.children.iter() }
                </div>
            }

            { actions }

        </div>
    )
}

// alert group

/// A group for [`Alert`]s
#[derive(Clone, PartialEq, Properties)]
pub struct GroupProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<Alert>,
    #[prop_or_default]
    pub toast: bool,
}

#[function_component(AlertGroup)]
pub fn view(props: &GroupProperties) -> Html {
    let mut classes = classes!("pf-c-alert-group");

    if props.toast {
        classes.push(classes!("pf-m-toast"));
    }

    html! (
        <ul class={classes}>
            { for props.children.iter().map(|child|html!{
                <li class="pf-c-alert-group__item">
                    { child }
                </li>
            })}
        </ul>
    )
}
