#[cfg(feature = "router")]
mod router;
#[cfg(feature = "router")]
pub use router::*;

use crate::Icon;
use std::fmt::Debug;
use yew::prelude::*;

// nav

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NavProps {
    #[prop_or_default]
    pub children: Children,
}

/// A navigation component.
pub struct Nav {
    props: NavProps,
}

impl Component for Nav {
    type Message = ();
    type Properties = NavProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <nav class="pf-c-nav" aria-label="Global">
                { for self.props.children.iter() }
            </nav>
        }
    }
}

// nav list

#[derive(Clone, PartialEq, Properties)]
pub struct NavListProps {
    #[prop_or_default]
    pub children: Children,
}

pub struct NavList {
    props: NavListProps,
}

impl Component for NavList {
    type Message = ();
    type Properties = NavListProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <ul class="pf-c-nav__list">
                { for self.props.children.iter() }
            </ul>
        }
    }
}

// nav group

#[derive(Clone, PartialEq, Properties)]
pub struct NavGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
}

/// Navigation group/section.
pub struct NavGroup {
    props: NavGroupProps,
}

impl Component for NavGroup {
    type Message = ();
    type Properties = NavGroupProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        html! {
            <section class="pf-c-nav__section">
                <h2 class="pf-c-nav__section-title">{ self.props.title.clone() }</h2>
                <NavList>
                    { for self.props.children.iter() }
                </NavList>
            </section>
        }
    }
}

// nav item

#[derive(Clone, PartialEq, Properties)]
pub struct NavItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub to: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub external: bool,
}

/// A navigation item (link).
pub struct NavItem {
    props: NavItemProps,
}

impl Component for NavItem {
    type Message = ();
    type Properties = NavItemProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        false
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
        let mut target = self.props.target.as_str();
        if target.is_empty() && self.props.external {
            target = "_blank";
        }

        return html! {
            <li class="pf-c-nav__item">
                <a
                    href=self.get_href()
                    class="pf-c-nav__link"
                    target=target
                >
                    { for self.props.children.iter() }
                    { if self.props.external {html!{
                        <span class="pf-u-ml-sm pf-u-font-size-sm">{Icon::ExternalLinkAlt}</span>
                    }} else {html!{}}}
                </a>
            </li>
        };
    }
}

impl NavItem {
    fn get_href(&self) -> String {
        if self.props.to.is_empty() {
            "#".into()
        } else {
            self.props.to.clone()
        }
    }
}

// nav group

#[derive(Clone, PartialEq, Properties)]
pub struct NavExpandableProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub expanded: bool,
}

/// Expandable navigation group/section.
pub struct NavExpandable {
    props: NavExpandableProps,
    link: ComponentLink<Self>,

    expanded: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum MsgExpandable {
    Toggle,
}

impl Component for NavExpandable {
    type Message = MsgExpandable;
    type Properties = NavExpandableProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let expanded = match props.expanded {
            true => Some(true),
            false => None,
        };

        Self {
            props,
            link,
            expanded,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            MsgExpandable::Toggle => {
                self.expanded = Some(!self.is_expanded());
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            if self.props.expanded {
                self.expanded = Some(true);
            }
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-nav__item pf-c-expandable");

        let expanded = self.is_expanded();

        if expanded {
            classes.push("pf-m-expanded");
        }

        return html! {
            <li class=classes>
                <button
                    class="pf-c-nav__link"
                    aria-expanded=expanded
                    onclick=self.link.callback(|_|MsgExpandable::Toggle)
                    >
                    { &self.props.title }
                    <span class="pf-c-nav__toggle">
                        <span class="pf-c-nav__toggle-icon">
                            { Icon::AngleRight }
                        </span>
                    </span>
                </button>

                <section class="pf-c-nav__subnav" hidden=!expanded>
                    <NavList>
                        { for self.props.children.iter() }
                    </NavList>
                </section>
            </li>
        };
    }
}

impl NavExpandable {
    fn is_expanded(&self) -> bool {
        self.expanded.unwrap_or(self.props.expanded)
    }
}
