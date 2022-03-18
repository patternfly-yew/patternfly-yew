use super::*;
use std::collections::HashMap;
use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use yew::html::ChildrenRenderer;
use yew::prelude::*;
use yew::virtual_dom::{VChild, VComp};
use yew_router::agent::RouteRequest::GetCurrentRoute;
use yew_router::prelude::*;

// nav router item

#[derive(Clone, PartialEq, Properties)]
pub struct NavRouterItemProps<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    #[prop_or_default]
    pub children: Children,
    pub to: R,
    #[prop_or_default]
    pub active: bool,

    #[prop_or_default]
    pub on_active: Callback<bool>,
}

/// A navigation item, using the Router.
pub struct NavRouterItem<R>
where
    R: Switch + Debug,
{
    active: bool,
    _router: RouteAgentBridge,
    _marker: PhantomData<R>,
}

#[derive(Clone)]
pub enum NavRouterMsg<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    RouteChange(Option<R>),
}

impl<R> Component for NavRouterItem<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
{
    type Message = NavRouterMsg<R>;
    type Properties = NavRouterItemProps<R>;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|route: yew_router::route::Route| {
            NavRouterMsg::RouteChange(Switch::switch(route))
        });
        let active = ctx.props().active;
        let mut _router = RouteAgentBridge::new(callback);
        _router.send(GetCurrentRoute);
        Self {
            active,
            _router,
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            NavRouterMsg::RouteChange(ref route) => {
                self.active = route
                    .as_ref()
                    .map(|sw| sw == &ctx.props().to)
                    .unwrap_or_default();

                ctx.props().on_active.emit(self.active);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-nav__link");

        if self.active {
            classes.push("pf-m-current");
        }

        html! {
            <li class="pf-c-nav__item">
                <RouterAnchor<R> route={ctx.props().to.clone()} classes={classes.to_string()}>
                    { for ctx.props().children.iter() }
                </RouterAnchor<R>>
            </li>
        }
    }
}

// nav router group children

#[derive(Clone, PartialEq)]
pub enum NavRouterExpandableChild<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
{
    Item(Rc<<NavItem as Component>::Properties>),
    RouterItem(Rc<<NavRouterItem<R> as Component>::Properties>),
}

impl<R> NavRouterExpandableChild<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    fn set_on_active(&mut self, callback: Callback<bool>) {
        match self {
            Self::RouterItem(props) => {
                let props = Rc::make_mut(props);
                props.on_active = callback
            }
            _ => {}
        }
    }
}

impl<R> From<NavItemProps> for NavRouterExpandableChild<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    fn from(props: NavItemProps) -> Self {
        NavRouterExpandableChild::Item(Rc::new(props))
    }
}

impl<R> From<NavRouterItemProps<R>> for NavRouterExpandableChild<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    fn from(props: NavRouterItemProps<R>) -> Self {
        NavRouterExpandableChild::RouterItem(Rc::new(props))
    }
}

// nav router group variant

#[derive(PartialEq, Clone)]
pub struct NavRouterExpandableVariant<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
{
    props: NavRouterExpandableChild<R>,
}

impl<R, CHILD> From<VChild<CHILD>> for NavRouterExpandableVariant<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
    CHILD: Component,
    CHILD::Properties: Into<NavRouterExpandableChild<R>> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl<R> Into<Html> for NavRouterExpandableVariant<R>
where
    R: Switch + PartialEq + Clone + Debug,
{
    fn into(self) -> Html {
        match self.props {
            NavRouterExpandableChild::Item(props) => {
                VComp::new::<NavItem>(props, NodeRef::default(), None).into()
            }
            NavRouterExpandableChild::RouterItem(props) => {
                VComp::new::<NavRouterItem<R>>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// nav router group

#[derive(Clone, PartialEq, Properties)]
pub struct NavRouterExpandableProps<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
{
    #[prop_or_default]
    pub children: ChildrenRenderer<NavRouterExpandableVariant<R>>,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub expanded: bool,
}

/// A navigation item, using the Router.
pub struct NavRouterExpandable<R>
where
    R: Switch + Debug,
{
    state: HashMap<usize, bool>,
    active: bool,
    _marker: PhantomData<R>,
}

#[derive(Clone)]
pub enum NavRouterExpandableMsg {
    ChildActive(usize, bool),
}

impl<R> Component for NavRouterExpandable<R>
where
    R: Switch + PartialEq + Clone + Debug + 'static,
{
    type Message = NavRouterExpandableMsg;
    type Properties = NavRouterExpandableProps<R>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            state: HashMap::new(),
            active: false,
            _marker: Default::default(),
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Self::Message::ChildActive(idx, active) => {
                self.state.insert(idx, active);
                self.active = self.state.iter().any(|(_, v)| *v);
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let expanded = self.active || ctx.props().expanded;

        return html! {
            <NavExpandable
                title={ctx.props().title.clone()}
                expanded={expanded}
                >
                { for ctx.props().children.iter().enumerate().map(|(i, mut c)|{
                    let on_active = ctx
                        .link()
                        .callback(move |active| NavRouterExpandableMsg::ChildActive(i, active));
                    c.props.set_on_active(on_active);
                    c
                }) }
            </NavExpandable>
        };
    }
}
