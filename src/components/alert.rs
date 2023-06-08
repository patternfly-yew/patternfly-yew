//! Alert popup

use crate::{Action, Button, ButtonVariant, Icon};

use yew::prelude::*;

#[deprecated(since = "0.4.0", note = "This type has been renamed to 'AlertType'")]
pub type Type = AlertType;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum AlertType {
    #[default]
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertType {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            AlertType::Default => vec![],
            AlertType::Info => vec!["pf-m-info"],
            AlertType::Success => vec!["pf-m-success"],
            AlertType::Warning => vec!["pf-m-warning"],
            AlertType::Danger => vec!["pf-m-danger"],
        }
    }

    pub fn aria_label(&self) -> &'static str {
        match self {
            AlertType::Default => "Default alert",
            AlertType::Info => "Information alert",
            AlertType::Success => "Success alert",
            AlertType::Warning => "Warning alert",
            AlertType::Danger => "Danger alert",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            AlertType::Default => Icon::Bell,
            AlertType::Info => Icon::InfoCircle,
            AlertType::Success => Icon::CheckCircle,
            AlertType::Warning => Icon::ExclamationTriangle,
            AlertType::Danger => Icon::ExclamationCircle,
        }
    }
}

/// Properties for [`Alert`]
#[derive(Clone, PartialEq, Properties)]
pub struct AlertProperties {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub r#type: AlertType,
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
    let mut classes = classes!("pf-v5-c-alert");

    classes.extend(props.r#type.as_classes());

    if props.inline {
        classes.push("pf-m-inline");
    }

    let mut title_classes = classes!("pf-v5-c-alert__title");

    if props.truncate {
        title_classes.push("pf-m-truncate");
    }

    let t = props.r#type;

    let actions = if props.actions.is_empty() {
        html!()
    } else {
        html! (
            <div class="pf-v5-c-alert__action-group">
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
            <div class="pf-v5-c-alert__icon">{ t.icon() }</div>
            <p class={title_classes}>
                <span class="pf-v5-screen-reader">{ t.aria_label() }{":"}</span>
                { &props.title }
            </p>


            if let Some(onclose) = props.onclose.as_ref() {
                <div class="pf-v5-c-alert__action">
                    <Button variant={ButtonVariant::Plain} icon={Icon::Times} onclick={onclose.clone().reform(|_|())} />
                </div>
            }


            if !props.children.is_empty() {
                <div class="pf-v5-c-alert__description">
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
    let mut classes = classes!("pf-v5-c-alert-group");

    if props.toast {
        classes.push(classes!("pf-m-toast"));
    }

    html! (
        <ul class={classes} role="list">
            { for props.children.iter().map(|child|html!{
                <li class="pf-v5-c-alert-group__item">
                    { child }
                </li>
            })}
        </ul>
    )
}
