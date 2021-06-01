use std::fmt::Debug;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::agent::RouteRequest::GetCurrentRoute;
use yew_router::agent::*;
use yew_router::Switch;

pub struct SwitchTransformer<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    from: Box<dyn Fn(&GSWITCH) -> Option<&LSWITCH>>,
    to: Box<dyn Fn(LSWITCH) -> GSWITCH>,
}

impl<GSWITCH, LSWITCH> SwitchTransformer<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    pub fn new<FROM, TO>(from: FROM, to: TO) -> Rc<Self>
    where
        FROM: Fn(&GSWITCH) -> Option<&LSWITCH> + 'static,
        TO: Fn(LSWITCH) -> GSWITCH + 'static,
    {
        Rc::new(Self {
            from: Box::new(from),
            to: Box::new(to),
        })
    }
}

// tab router

#[derive(Properties, Clone)]
pub struct Props<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    pub transformer: Rc<SwitchTransformer<GSWITCH, LSWITCH>>,

    #[prop_or_default]
    pub r#box: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub filled: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<TabRouterItem<LSWITCH>>,
}

#[derive(Clone, Debug)]
pub enum TabsRouterMsg<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    RouteChange(Option<GSWITCH>),
    Select(LSWITCH),
}

pub struct TabsRouter<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    props: Props<GSWITCH, LSWITCH>,
    link: ComponentLink<Self>,
    _router: RouteAgentBridge,

    active: Option<GSWITCH>,
}

impl<GSWITCH, LSWITCH> Component for TabsRouter<GSWITCH, LSWITCH>
where
    GSWITCH: 'static + Switch + Clone + PartialEq + Debug,
    LSWITCH: 'static + Clone + PartialEq + Debug,
{
    type Message = TabsRouterMsg<GSWITCH, LSWITCH>;
    type Properties = Props<GSWITCH, LSWITCH>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(|route: yew_router::route::Route| {
            TabsRouterMsg::RouteChange(Switch::switch(route))
        });
        let mut bridge = RouteAgentBridge::new(callback);
        bridge.send(GetCurrentRoute);
        Self {
            props,
            link,
            active: None,
            _router: bridge,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TabsRouterMsg::RouteChange(route) => {
                self.active = route;
                true
            }
            TabsRouterMsg::Select(route) => {
                let route = (self.props.transformer.to)(route);
                RouteAgentDispatcher::<()>::new().send(RouteRequest::ChangeRoute(route.into()));
                false
            }
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-tabs");

        if self.props.r#box {
            classes = classes.extend("pf-m-box");
        }

        if self.props.vertical {
            classes = classes.extend("pf-m-vertical");
        }

        if self.props.filled {
            classes = classes.extend("pf-m-fill");
        }

        let local = self
            .active
            .as_ref()
            .and_then(|active| (self.props.transformer.from)(active));

        return html! {
            <div class=classes>
                <ul class="pf-c-tabs__list">
                    { for self.props.children.iter().map(|mut c|{
                        let to = c.props.to.clone();
                        c.props.current = local.map(|local| *local == c.props.to).unwrap_or_default();
                        c.props.onselect = self.link.callback(move |_|TabsRouterMsg::Select(to.clone()));
                        c
                    }) }
                </ul>
            </div>
        };
    }
}

// tab router item

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TabRouterItemProps<SWITCH>
where
    SWITCH: 'static + Clone + Debug + PartialEq,
{
    /// The tab label
    pub label: String,
    /// The switch this item references to
    pub to: SWITCH,
    /// If the item is currently selected
    #[prop_or_default]
    pub(crate) current: bool,
    #[prop_or_default]
    pub(crate) onselect: Callback<()>,
}

#[derive(Clone, Copy, Debug)]
pub enum TabRouterItemMsg {
    Clicked,
}

pub struct TabRouterItem<SWITCH>
where
    SWITCH: 'static + Clone + Debug + PartialEq,
{
    props: TabRouterItemProps<SWITCH>,
    link: ComponentLink<Self>,
}

impl<SWITCH> Component for TabRouterItem<SWITCH>
where
    SWITCH: 'static + Clone + Debug + PartialEq,
{
    type Message = TabRouterItemMsg;
    type Properties = TabRouterItemProps<SWITCH>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            TabRouterItemMsg::Clicked => {
                self.props.onselect.emit(());
            }
        }
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
        let mut classes = Classes::from("pf-c-tabs__item");

        if self.props.current {
            classes = classes.extend("pf-m-current");
        }

        return html! {
            <li class=classes>
                <button class="pf-c-tabs__link" onclick=self.link.callback(|_|TabRouterItemMsg::Clicked)>
                    <span class="pf-c-tabs__item-text"> { &self.props.label } </span>
                </button>
            </li>
        };
    }
}
