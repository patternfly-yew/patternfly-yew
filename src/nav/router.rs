use super::*;
use std::fmt::Debug;
use yew::prelude::*;

use std::collections::HashMap;

use yew::html::ChildrenRenderer;
use yew::virtual_dom::{VChild, VComp};
use yew_router::{
    agent::RouteRequest::GetCurrentRoute, components::RouterAnchor, prelude::RouteAgentBridge,
    Switch,
};

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

// nav router group children

#[derive(Clone, PartialEq)]
pub enum NavRouterExpandableChild<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    Item(<NavItem as Component>::Properties),
    RouterItem(<NavRouterItem<SWITCH> as Component>::Properties),
}

impl<SWITCH> NavRouterExpandableChild<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    fn set_on_active(&mut self, callback: Callback<bool>) {
        match self {
            Self::RouterItem(props) => props.on_active = callback,
            _ => {}
        }
    }
}

impl<SWITCH> From<NavItemProps> for NavRouterExpandableChild<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    fn from(props: NavItemProps) -> Self {
        NavRouterExpandableChild::Item(props)
    }
}

impl<SWITCH> From<NavRouterItemProps<SWITCH>> for NavRouterExpandableChild<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    fn from(props: NavRouterItemProps<SWITCH>) -> Self {
        NavRouterExpandableChild::RouterItem(props)
    }
}

// nav router group variant

#[derive(PartialEq, Clone)]
pub struct NavRouterExpandableVariant<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    props: NavRouterExpandableChild<SWITCH>,
}

impl<SWITCH, CHILD> From<VChild<CHILD>> for NavRouterExpandableVariant<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
    CHILD: Component,
    CHILD::Properties: Into<NavRouterExpandableChild<SWITCH>>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl<SWITCH> Into<Html> for NavRouterExpandableVariant<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    fn into(self) -> Html {
        match self.props {
            NavRouterExpandableChild::Item(props) => {
                VComp::new::<NavItem>(props, NodeRef::default(), None).into()
            }
            NavRouterExpandableChild::RouterItem(props) => {
                VComp::new::<NavRouterItem<SWITCH>>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// nav router group

#[derive(Clone, Properties)]
pub struct NavRouterExpandableProps<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    #[prop_or_default]
    pub children: ChildrenRenderer<NavRouterExpandableVariant<SWITCH>>,
    #[prop_or_default]
    pub title: String,
}

/// A navigation item, using the Router.
pub struct NavRouterExpandable<SWITCH>
where
    SWITCH: Switch + Clone + PartialEq + Debug + 'static,
{
    props: NavRouterExpandableProps<SWITCH>,
    link: ComponentLink<Self>,

    state: HashMap<usize, bool>,
    active: bool,
}

#[derive(Clone)]
pub enum NavRouterExpandableMsg {
    ChildActive(usize, bool),
}

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
                    let on_active = self
                        .link
                        .callback(move |active| NavRouterExpandableMsg::ChildActive(i, active));
                    c.props.set_on_active(on_active);
                    c
                }) }
            </NavExpandable>
        };
    }
}
