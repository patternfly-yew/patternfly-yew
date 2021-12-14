use crate::{Action, Button, Icon, Variant};

use yew::prelude::*;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum Type {
    Default,
    Info,
    Success,
    Warning,
    Danger,
}

impl Default for Type {
    fn default() -> Self {
        Self::Default
    }
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
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

pub struct Alert {}

impl Component for Alert {
    type Message = ();
    type Properties = Props;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-alert");

        classes.extend(ctx.props().r#type.as_classes());

        if ctx.props().inline {
            classes.push("pf-m-inline");
        }

        if ctx.props().truncate {
            classes.push("pf-m-truncate");
        }

        let t = ctx.props().r#type;

        let actions = if ctx.props().actions.is_empty() {
            html! {}
        } else {
            html! {
                <div class="pf-c-alert__action-group">
                    {for ctx.props().actions.iter().map(|action|{
                        html!{
                            <Button
                                variant={Variant::InlineLink}
                                label={action.label.clone()}
                                onclick={action.callback.reform(|_|())}
                            />
                        }
                    })}
                </div>
            }
        };

        return html! {
            <div id={ctx.props().id.clone()} class={classes} aria_label={t.aria_label()}>
                <div class="pf-c-alert__icon">{ t.icon() }</div>
                <div class="pf-c-alert__title">
                    <strong>
                        <span class="pf-screen-reader">{ t.aria_label() }{":"}</span>
                        { &ctx.props().title }
                    </strong>
                </div>

                {
                    if let Some(onclose) = ctx.props().onclose.as_ref() {
                        html!{
                            <div class="pf-c-alert__action">
                                <Button variant={Variant::Plain} icon={Icon::Times} onclick={onclose.clone().reform(|_|())} />
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }

                {
                    if ctx.props().children.len() > 0 {
                        html!{
                            <div class="pf-c-alert__description">
                                { for ctx.props().children.iter() }
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }

                { actions }

            </div>
        };
    }
}

// alert group

#[derive(Clone, PartialEq, Properties)]
pub struct GroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Alert>,
    #[prop_or_default]
    pub toast: bool,
}

pub struct AlertGroup {}

impl Component for AlertGroup {
    type Message = ();
    type Properties = GroupProps;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-alert-group");

        if ctx.props().toast {
            classes.push("pf-m-toast");
        }

        return html! {
            <ul class={classes}>
                { for ctx.props().children.iter().map(|child|html!{
                    <li class="pf-c-alert-group__item">
                        { child }
                    </li>
                })}
            </ul>
        };
    }
}
