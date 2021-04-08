use crate::Icon;
use std::fmt::Debug;
use yew::prelude::*;

const LOG_TARGET: &'static str = "nav";

use std::collections::HashMap;

#[cfg(feature = "router")]
use yew_router::{
    agent::RouteRequest::GetCurrentRoute, components::RouterAnchor, prelude::RouteAgentBridge,
    Switch,
};

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
        html! {
            <li class="pf-c-nav__item">
                <a
                    href=self.get_href()
                    class="pf-c-nav__link"
                    target=self.props.target
                >
                    { for self.props.children.iter() }
                </a>
            </li>
        }
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

// nav router item

#[cfg(feature = "router")]
#[derive(Clone, PartialEq, Properties)]
pub struct NavRouterItemProps<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    #[prop_or_default]
    pub children: Children,
    pub to: SWITCH,
    #[prop_or_default]
    pub active: bool,

    #[prop_or_default]
    pub on_active: Callback<bool>,
}

/// A navigation item, using the Router.
#[cfg(feature = "router")]
pub struct NavRouterItem<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    props: NavRouterItemProps<SWITCH>,
    active: bool,
    _router: RouteAgentBridge,
}

#[cfg(feature = "router")]
#[derive(Clone)]
pub enum NavRouterMsg<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    RouteChange(Option<SWITCH>),
}

#[cfg(feature = "router")]
impl<SWITCH> Component for NavRouterItem<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    type Message = NavRouterMsg<SWITCH>;
    type Properties = NavRouterItemProps<SWITCH>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|route: yew_router::route::Route| {
            NavRouterMsg::RouteChange(Switch::switch(route))
        });
        let active = props.active;
        let mut bridge = RouteAgentBridge::new(callback);
        bridge.send(GetCurrentRoute);
        Self {
            props,
            active,
            _router: bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            NavRouterMsg::RouteChange(ref route) => {
                log::debug!(
                    target: LOG_TARGET,
                    "Route change: {:?} {} {}",
                    self.props.to,
                    route.is_some(),
                    route
                        .as_ref()
                        .map(|s| s == &self.props.to)
                        .map(|s| s.to_string())
                        .unwrap_or_else(|| "<none>".into())
                );
                self.active = route
                    .as_ref()
                    .map(|sw| sw == &self.props.to)
                    .unwrap_or_default();

                self.props.on_active.emit(self.active);
            }
        }
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
        let mut classes = Classes::from("pf-c-nav__link");

        if self.active {
            classes.push("pf-m-current");
        }

        return html! {
            <li class="pf-c-nav__item">
                <RouterAnchor<SWITCH> route=self.props.to.clone() classes=classes.to_string()>
                    { for self.props.children.iter() }
                </RouterAnchor<SWITCH>>
            </li>
        };
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

// nav router group

#[cfg(feature = "router")]
#[derive(Clone, Properties)]
pub struct NavRouterExpandableProps<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    #[prop_or_default]
    pub children: ChildrenWithProps<NavRouterItem<SWITCH>>,
    #[prop_or_default]
    pub title: String,
}

/// A navigation item, using the Router.
#[cfg(feature = "router")]
pub struct NavRouterExpandable<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    props: NavRouterExpandableProps<SWITCH>,
    link: ComponentLink<Self>,

    state: HashMap<usize, bool>,
    active: bool,
}

#[cfg(feature = "router")]
#[derive(Clone)]
pub enum NavRouterExpandableMsg {
    ChildActive(usize, bool),
}

#[cfg(feature = "router")]
impl<SWITCH> Component for NavRouterExpandable<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    type Message = NavRouterExpandableMsg;
    type Properties = NavRouterExpandableProps<SWITCH>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            props,
            link,
            state: HashMap::new(),
            active: false,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Self::Message::ChildActive(idx, active) => {
                self.state.insert(idx, active);
                self.active = self.state.iter().any(|(_, v)| *v);
            }
        }
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        return html! {
            <NavExpandable
                title=&self.props.title
                expanded=&self.active
                >
                { for self.props.children.iter().enumerate().map(|(i, mut c)|{
                    c.props.on_active = self
            .link
            .callback(move |active| NavRouterExpandableMsg::ChildActive(i, active));
                    c
                }) }
            </NavExpandable>
        };
    }
}
