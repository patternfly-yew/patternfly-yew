use yew::html::ChildrenRenderer;
use yew::{html, Component, ComponentLink, Html};
use yew::{Classes, Properties};

#[derive(Copy, Clone, Eq, PartialEq)]
pub enum PageSectionVariant {
    Default,
    Darker,
    Dark,
    Light,
}

impl Default for PageSectionVariant {
    fn default() -> Self {
        Self::Default
    }
}

impl PageSectionVariant {
    pub fn as_class(&self) -> Vec<String> {
        match self {
            Self::Default => vec![],
            Self::Darker => vec!["pf-m-dark-100".into()],
            Self::Dark => vec!["pf-m-dark-200".into()],
            Self::Light => vec!["pf-m-light".into()],
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenRenderer<Html>,
    #[prop_or_default]
    pub variant: PageSectionVariant,
    #[prop_or_default]
    pub fill: bool,
}

#[derive(Clone, PartialEq)]
pub struct PageSection {
    props: Props,
}

impl Component for PageSection {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> bool {
        /*
        match msg {
            _ => {}
        }
         */

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
        html! {
            <section class=self.collect_classes()>
            { for self.props.children.iter() }
            </section>
        }
    }
}

impl PageSection {
    fn collect_classes(&self) -> Classes {
        let mut classes = Classes::from("pf-c-page__main-section");
        classes = classes.extend(self.props.variant.as_class());

        if self.props.fill {
            classes.push("pf-m-fill");
        }

        classes
    }
}
