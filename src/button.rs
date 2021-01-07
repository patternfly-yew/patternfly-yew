use crate::Icon;
use yew::prelude::*;

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Variant {
    None,
    Primary,
    Secondary,
    Tertiary,
    Warning,
    Danger,
    Link,
    InlineLink,
    Control,
    Plain,
}

impl Variant {
    pub fn as_classes(&self) -> Vec<&str> {
        match self {
            Variant::None => vec![],
            Variant::Primary => vec!["pf-m-primary"],
            Variant::Secondary => vec!["pf-m-secondary"],
            Variant::Tertiary => vec!["pf-m-tertiary"],
            Variant::Warning => vec!["pf-m-warning"],
            Variant::Danger => vec!["pf-m-danger"],
            Variant::Link => vec!["pf-m-link"],
            Variant::InlineLink => vec!["pf-m-link", "pf-m-inline"],
            Variant::Control => vec!["pf-m-control"],
            Variant::Plain => vec!["pf-m-plain"],
        }
    }
}

impl Default for Variant {
    fn default() -> Self {
        Variant::None
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

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub id: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub onclick: Callback<yew::MouseEvent>,
    #[prop_or_default]
    pub variant: Variant,
    #[prop_or_default]
    pub icon: Option<Icon>,
    #[prop_or_default]
    pub align: Align,

    #[prop_or_default]
    pub aria_label: Option<String>,

    #[prop_or("button".into())]
    pub r#type: String,
}

pub struct Button {
    props: Props,
}

impl Component for Button {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        true
    }

    fn change(&mut self, props: Self::Properties) -> bool {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-button");

        classes = classes.extend(self.props.variant.as_classes());

        return html! {
            <button
                id=&self.props.id
                class=classes
                type=self.props.r#type
                onclick=&self.props.onclick>

                { self.label() }

            </button>
        };
    }
}

impl Button {
    fn icon(&self) -> Html {
        let mut classes = Classes::from("pf-c-button__icon");

        match self.props.align {
            Align::Start => classes.push("pf-m-start"),
            Align::End => classes.push("pf-m-end"),
        }

        match self.props.icon {
            Some(i) => html! {
                <span class=classes>
                    { i }
                </span>
            },
            None => html! {},
        }
    }

    fn label(&self) -> Vec<Html> {
        let label = self.props.label.clone().into();
        match self.props.align {
            Align::Start => vec![self.icon(), label],
            Align::End => vec![label, self.icon()],
        }
    }
}
