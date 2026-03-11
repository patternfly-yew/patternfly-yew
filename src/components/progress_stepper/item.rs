use super::ProgressStepperStepStatus;
use crate::prelude::{Popover, PopoverBody};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ProgressStepperStepProperties {
    pub title: String,
    #[prop_or_default]
    pub description: Option<String>,
    #[prop_or_default]
    pub is_current: bool,
    pub status: ProgressStepperStepStatus,
    #[prop_or_default]
    pub icon: Option<Html>,
    #[prop_or_default]
    /// (header, body, footer)
    pub popover: Option<(Html, Html, Html)>,
}

#[component]
pub fn ProgressStepperStep(props: &ProgressStepperStepProperties) -> Html {
    let mut class = classes!("pf-v6-c-progress-stepper__step");

    let (status_class, icon_class) = match props.status {
        ProgressStepperStepStatus::Default => {
            ("", classes!("pf-v6-pficon", "pf-v6-pficon-resources-full"))
        }
        ProgressStepperStepStatus::Success => ("pf-m-success", classes!("fas", "fa-check-circle")),
        ProgressStepperStepStatus::Info => (
            "pf-m-info",
            classes!("pf-v6-pficon", "pf-v6-pficon-resources-full"),
        ),
        ProgressStepperStepStatus::Pending => ("pf-m-pending", classes!("pf-v6-pficon", "")),
        ProgressStepperStepStatus::Warning => {
            ("pf-m-warning", classes!("fas", "fa-exclamation-triangle"))
        }
        ProgressStepperStepStatus::Danger => {
            ("pf-m-danger", classes!("fas", "fa-exclamation-circle"))
        }
    };

    class.push(classes!(status_class));

    if props.is_current {
        class.push("pf-m-current");
    }

    let description = props.description.as_ref().map(|description|
        html!(<div class="pf-v6-c-progress-stepper__step-description">{ description }</div>)
    ).unwrap_or_default();

    let icon = props
        .icon
        .clone()
        .unwrap_or(html!(<i class={icon_class} aria-hidden="true" />));

    let title = if let Some((header, body, footer)) = props.popover.clone() {
        let target = html! { props.title.clone() };
        let body = html_nested! { <PopoverBody {header} {footer}>{ body }</PopoverBody> };

        html!(<Popover {target} {body} />)
    } else {
        html!(props.title.clone())
    };

    let mut title_class = classes!("pf-v6-c-progress-stepper__step-title");
    if props.popover.is_some() {
        title_class.push(classes!("pf-m-help-text"));
    }

    html! {
        <li {class}>
            <span class="pf-v6-c-progress-stepper__step-connector">
                <span class="pf-v6-c-progress-stepper__step-icon">{ icon }</span>
            </span>
            <span class="pf-v6-c-progress-stepper__step-main">
                <span class={title_class}>{ title }</span>
                { description }
            </span>
        </li>
    }
}
