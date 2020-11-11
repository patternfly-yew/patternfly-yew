use crate::{Button, Icon, Variant};

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
    pub fn as_classes(&self) -> Vec<&str> {
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
    pub r#type: Type,
    pub title: String,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub inline: bool,
    #[prop_or_default]
    pub truncate: bool,
    #[prop_or_default]
    pub onclose: Option<Callback<()>>,
}

pub struct Alert {
    props: Props,
}

impl Component for Alert {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-alert");

        classes = classes.extend(self.props.r#type.as_classes());

        if self.props.inline {
            classes.push("pf-m-inline");
        }

        if self.props.truncate {
            classes.push("pf-m-truncate");
        }

        let t = self.props.r#type;

        return html! {
            <div class=classes aria_label=t.aria_label()>
                <div class="pf-c-alert__icon">{ t.icon() }</div>
                <div class="pf-c-alert__title">
                    <strong>
                        <span class="pf-screen-reader">{t.aria_label()}{":"}</span>
                        { &self.props.title }
                    </strong>
                </div>

                {
                    if let Some(onclose) = self.props.onclose.as_ref() {
                        html!{
                            <Button variant=Variant::Plain icon=Icon::Times onclick=onclose.clone().reform(|_|())/>
                        }
                    } else {
                        html!{}
                    }
                }

                {
                    if self.props.children.len() > 0 {
                        html!{
                            <div class="pf-c-alert__description">
                                { for self.props.children.iter() }
                            </div>
                        }
                    } else {
                        html!{}
                    }
                }


            </div>
        };
    }
}

// alert group

#[derive(Clone, PartialEq, Properties)]
pub struct GroupProps {
    #[prop_or_default]
    pub children: ChildrenWithProps<Alert>,
}

pub struct AlertGroup {
    props: GroupProps,
}

impl Component for AlertGroup {
    type Message = ();
    type Properties = GroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        return html! {
            <ul class="pf-c-alert-group">
                { for self.props.children.iter().map(|child|html_nested!{
                    <li>
                        { child }
                    </li>
                })}
            </ul>
        };
    }
}
