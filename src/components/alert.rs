//! Alert popup

use crate::ouia;
use crate::prelude::wrap::wrapper_elt_with_attributes;
use crate::prelude::{Action, Button, ButtonVariant, Icon, Level, Tooltip, TooltipPosition};
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;

const OUIA: Ouia = ouia!("Alert");

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq, Ord, PartialOrd)]
pub enum AlertType {
    #[default]
    Custom,
    Info,
    Success,
    Warning,
    Danger,
}

impl AlertType {
    pub fn as_classes(&self) -> Vec<&'static str> {
        match self {
            AlertType::Custom => vec!["pf-m-custom"],
            AlertType::Info => vec!["pf-m-info"],
            AlertType::Success => vec!["pf-m-success"],
            AlertType::Warning => vec!["pf-m-warning"],
            AlertType::Danger => vec!["pf-m-danger"],
        }
    }

    pub fn aria_label(&self) -> &'static str {
        match self {
            AlertType::Custom => "Custom alert",
            AlertType::Info => "Information alert",
            AlertType::Success => "Success alert",
            AlertType::Warning => "Warning alert",
            AlertType::Danger => "Danger alert",
        }
    }

    pub fn icon(&self) -> Icon {
        match self {
            AlertType::Custom => Icon::Bell,
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
    pub title: Html,
    /// Heading level of the alert title.
    #[prop_or(Level::H4)]
    pub title_level: Level,
    /// Announce changes to this alert without interrupting the user.
    #[prop_or_default]
    pub live_region: bool,
    /// Localized variant prefix, without the trailing colon.
    #[prop_or_default]
    pub variant_label: Option<String>,
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub toggle_aria_label: Option<AttrValue>,
    #[prop_or_default]
    pub custom_icon: Option<Html>,
    /// Maximum title lines; zero disables line-count truncation.
    #[prop_or_default]
    pub truncate_title: usize,
    #[prop_or(TooltipPosition::Top)]
    pub tooltip_position: TooltipPosition,
    /// Arbitrary action links, including router links.
    #[prop_or_default]
    pub action_links: Html,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub plain: bool,
    #[prop_or_default]
    pub truncate: bool,
    #[prop_or_default]
    pub actions: Vec<Action>,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

/// Alert component
///
/// > An **alert** is a notification that provides brief information to the user without blocking their workflow.
///
/// See: <https://www.patternfly.org/components/alert>
///
/// ## Properties
///
/// Defined by [`AlertProperties`].
#[function_component(Alert)]
pub fn alert(props: &AlertProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let expanded = use_state_eq(|| false);
    let ontoggle = use_callback(expanded.clone(), |_, expanded| expanded.set(!**expanded));
    let title_id = crate::hooks::id::use_id(None);
    let close_id = crate::hooks::id::use_id(None);
    render_alert(props, &ouia_id, *expanded, ontoggle, &title_id, &close_id)
}

fn render_alert(
    props: &AlertProperties,
    ouia_id: &str,
    expanded: bool,
    ontoggle: Callback<MouseEvent>,
    title_id: &AttrValue,
    close_id: &AttrValue,
) -> Html {
    let mut classes = classes!("pf-v6-c-alert");
    if props.expandable {
        classes.push("pf-m-expandable");
        if expanded {
            classes.push("pf-m-expanded");
        }
    }

    classes.extend(props.r#type.as_classes());

    if props.inline {
        classes.push("pf-m-inline");
    }
    if props.plain {
        classes.push("pf-m-plain");
    }

    let mut title_classes = classes!("pf-v6-c-alert__title");

    if props.truncate || props.truncate_title > 0 {
        title_classes.push("pf-m-truncate");
    }

    let t = props.r#type;
    let variant_label = props.variant_label.as_deref().unwrap_or(t.aria_label());
    let element = match props.title_level {
        Level::H1 => "h1",
        Level::H2 => "h2",
        Level::H3 => "h3",
        Level::H4 => "h4",
        Level::H5 => "h5",
        Level::H6 => "h6",
    };
    let truncated = props.truncate || props.truncate_title > 0;
    let title = html! {
        <@{element}
            id={title_id.clone()}
            class={title_classes}
            style={truncated.then(|| format!("--pf-v6-c-alert__title--max-lines: {}", props.truncate_title.max(1)))}
            tabindex={truncated.then_some("0")}
        >
            <span class="pf-v6-screen-reader">{ variant_label }{ ": " }</span>
            { props.title.clone() }
        </@>
    };

    let actions = if props.actions.is_empty() && props.action_links == Html::default() {
        html!()
    } else {
        html! (
            <div class="pf-v6-c-alert__action-group">
                { for props.actions.iter().map(|action|{
                    html!{
                        <Button
                            variant={ButtonVariant::InlineLink}
                            label={action.label.clone()}
                            onclick={action.callback.reform(|_|())}
                        />
                    }
                }) }
                { props.action_links.clone() }
            </div>
        )
    };

    html! (
        <div
            id={props.id.clone()}
            class={classes}
            aria-live={props.live_region.then_some("polite")}
            aria-atomic={props.live_region.then_some("false")}
            data-ouia-component-id={ouia_id.to_owned()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            if props.expandable {
                <div class="pf-v6-c-alert__toggle">
                    <Button
                        variant={ButtonVariant::Plain}
                        onclick={ontoggle}
                        aria_label={props.toggle_aria_label.clone().unwrap_or_else(|| format!("{variant_label} details").into())}
                        aria_expanded={expanded.to_string()}
                    >
                        <span class="pf-v6-c-alert__toggle-icon">{ Icon::AngleRight }</span>
                    </Button>
                </div>
            }
            <div
                class="pf-v6-c-alert__icon"
                aria-hidden="true"
            >
                { props.custom_icon.clone().unwrap_or_else(|| t.icon().into()) }
            </div>
            if truncated {
                <Tooltip
                    attach_to_child=true
                    content={props.title.clone()}
                    position={props.tooltip_position}
                >
                    { title }
                </Tooltip>
            } else {
                { title }
            }
            if let Some(onclose) = props.onclose.as_ref() {
                <div class="pf-v6-c-alert__action">
                    <span id={close_id.clone()} class="pf-v6-screen-reader">{ "Close" }</span>
                    <Button
                        variant={ButtonVariant::Plain}
                        icon={Icon::Times}
                        aria_labelledby={format!("{close_id} {title_id}")}
                        onclick={onclose.clone().reform(|_|())}
                    />
                </div>
            }
            if !props.children.is_empty() && (!props.expandable || expanded) {
                <div class="pf-v6-c-alert__description">{ props.children.clone() }</div>
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
    #[prop_or_default]
    pub live_region: bool,
}

#[function_component(AlertGroup)]
pub fn view(props: &GroupProperties) -> Html {
    let mut classes = classes!("pf-v6-c-alert-group");

    if props.toast {
        classes.push(classes!("pf-m-toast"));
    }

    html! (
        <ul
            class={classes}
            role="list"
            aria-live={props.live_region.then_some("polite")}
            aria-atomic={props.live_region.then_some("false")}
        >
            { for props.children.iter().map(|child|
                wrapper_elt_with_attributes(child.into(), "li", &[("class", AttributeOrProperty::Static("pf-v6-c-alert-group__item"))])
            ) }
        </ul>
    )
}

/// An anchor action, or a container for an arbitrary link (for example a router link).
/// When using children without `href`, give the child link the classes
/// `pf-v6-c-button pf-m-link pf-m-inline`.
#[derive(Clone, PartialEq, Properties)]
pub struct AlertActionLinkProperties {
    #[prop_or_default]
    pub href: Option<AttrValue>,
    #[prop_or_default]
    pub children: Html,
}

#[function_component(AlertActionLink)]
pub fn action_link(props: &AlertActionLinkProperties) -> Html {
    if let Some(href) = &props.href {
        html!(
            <a href={href.clone()} class="pf-v6-c-button pf-m-link pf-m-inline">
                { props.children.clone() }
            </a>
        )
    } else {
        props.children.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::render_html;

    #[test]
    fn default_and_live_markup_preserve_string_titles_and_actions() {
        let output = render_html(|| {
            html! {
                <Alert
                    title={String::from("Processing")}
                    inline=true
                    plain=true
                    live_region=true
                    onclose={Callback::noop()}
                    actions={vec![Action { label: "Retry".into(), callback: Callback::noop() }]}
                    action_links={html!(<AlertActionLink href="/status">{"View status"}</AlertActionLink>)}
                >
                    { "Details" }
                </Alert>
            }
        });
        for expected in [
            "pf-m-inline",
            "pf-m-plain",
            "aria-live=\"polite\"",
            "aria-atomic=\"false\"",
            "<h4",
            "Custom alert: ",
            "Processing",
            "pf-v6-c-alert__description",
            "pf-v6-c-alert__action-group",
            "href=\"/status\"",
            "Retry",
            "aria-labelledby=",
            ">Close</span>",
        ] {
            assert!(output.contains(expected), "Missing {expected}: {output}");
        }
        assert!(!output.contains("aria_label="));
        let output = render_html(|| html!(<Alert title="Static" />));
        assert!(!output.contains("aria-live"));
        assert!(!output.contains("aria-atomic"));
        assert!(!output.contains("pf-v6-c-alert__action-group"));
    }

    #[test]
    fn expandable_alert_hides_and_reveals_description() {
        for expanded in [false, true] {
            let output = render_html(move || {
                let props = yew::props!(AlertProperties {
                    title: html!(<em>{"Ready"}</em>),
                    title_level: Level::H2,
                    variant_label: "Bereit",
                    expandable: true,
                    toggle_aria_label: "Details anzeigen",
                    custom_icon: html!(<span>{"Custom icon"}</span>),
                    children: html!({ "Hidden details" }),
                });
                render_alert(
                    &props,
                    "test",
                    expanded,
                    Callback::noop(),
                    &"title".into(),
                    &"close".into(),
                )
            });
            assert!(output.contains("<h2"));
            assert!(output.contains("<em>Ready</em>"));
            assert!(output.contains("Bereit: "));
            assert!(output.contains("Custom icon"));
            assert!(output.contains("pf-m-expandable"));
            assert!(output.contains("aria-label=\"Details anzeigen\""));
            assert!(output.contains(&format!("aria-expanded=\"{expanded}\"")));
            assert_eq!(output.contains("Hidden details"), expanded);
            assert_eq!(output.contains("pf-m-expanded"), expanded);
        }
    }

    #[test]
    fn alert_group_is_an_optional_live_list() {
        for live_region in [false, true] {
            let output = render_html(move || {
                html! {
                    <AlertGroup {live_region} toast=true>
                        <Alert title="New notification" />
                    </AlertGroup>
                }
            });
            assert!(output.contains("<ul"));
            assert!(output.contains("role=\"list\""));
            assert!(output.contains("<li class=\"pf-v6-c-alert-group__item\""));
            assert!(output.contains("pf-m-toast"));
            assert_eq!(output.contains("aria-live=\"polite\""), live_region);
            assert_eq!(output.contains("aria-atomic=\"false\""), live_region);
        }
    }

    #[test]
    fn arbitrary_action_children_are_not_wrapped_in_a_button() {
        let output = render_html(|| {
            html! {
                <AlertActionLink>
                    <a href="/router-target">{ "Router link" }</a>
                </AlertActionLink>
            }
        });
        assert_eq!(output, "<a href=\"/router-target\">Router link</a>");
    }

    #[test]
    fn truncation_preserves_heading_and_sets_line_count() {
        for lines in [0, 3] {
            let output = render_html(move || {
                html! {
                    <Alert
                        title={html!(<em>{"Long title"}</em>)}
                        truncate={lines == 0}
                        truncate_title={lines}
                    />
                }
            });
            assert!(output.contains("pf-m-truncate"));
            assert!(output.contains(&format!(
                "--pf-v6-c-alert__title--max-lines: {}",
                lines.max(1)
            )));
            assert!(output.contains("tabindex=\"0\""));
            assert!(output.contains("aria-describedby="));
            assert!(output.contains("<h4"));
            assert!(!output.contains("<span><h4"));
        }
    }
}
