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
    pub align: Option<Align>,

    #[prop_or_default]
    pub aria_label: Option<String>,
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
                type="button"
                onclick=self.props.onclick.clone()>

                { self.icon() }
                { self.props.label.clone() }

            </button>
        };
    }
}

impl Button {
    pub fn icon(&self) -> Html {
        let mut classes = Classes::from("pf-c-button__icon");

        match self.props.align {
            Some(Align::Start) => classes.push("pf-m-start"),
            Some(Align::End) => classes.push("pf-m-end"),
            None => {}
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
}
