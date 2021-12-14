use std::fmt::Debug;
use std::marker::PhantomData;
use std::rc::Rc;
use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;

pub struct SwitchTransformer<GR, LR>
where
    GR: Switch + PartialEq + Debug,
    LR: 'static + Clone + PartialEq + Debug,
{
    from: Box<dyn Fn(&GR) -> Option<&LR>>,
    to: Box<dyn Fn(LR) -> GR>,
}

impl<GR, LR> SwitchTransformer<GR, LR>
where
    GR: Switch + PartialEq + Debug,
    LR: 'static + Clone + PartialEq + Debug,
{
    pub fn new<FROM, TO>(from: FROM, to: TO) -> Rc<Self>
    where
        FROM: Fn(&GR) -> Option<&LR> + 'static,
        TO: Fn(LR) -> GR + 'static,
    {
        Rc::new(Self {
            from: Box::new(from),
            to: Box::new(to),
        })
    }
}

// tab router

#[derive(Clone, Properties)]
pub struct Props<GR, LR>
where
    GR: Switch + PartialEq + Clone + Debug + 'static,
    LR: Switch + PartialEq + Clone + Debug + 'static,
{
    pub transformer: Rc<SwitchTransformer<GR, LR>>,

    #[prop_or_default]
    pub r#box: bool,
    #[prop_or_default]
    pub vertical: bool,
    #[prop_or_default]
    pub filled: bool,
    #[prop_or_default]
    pub children: ChildrenWithProps<TabRouterItem<LR>>,
}

impl<GR, LR> PartialEq for Props<GR, LR>
where
    GR: Switch + PartialEq + Clone + Debug + 'static,
    LR: Switch + PartialEq + Clone + Debug + 'static,
{
    fn eq(&self, other: &Self) -> bool {
        self.r#box == other.r#box
            && self.vertical == other.vertical
            && self.filled == other.filled
            && self.children == other.children
            && Rc::ptr_eq(&self.transformer, &other.transformer)
    }
}

#[derive(Clone, Debug)]
pub enum TabsRouterMsg<GR, LR>
where
    GR: Switch + Debug,
    LR: Switch + Debug,
{
    RouteChange(Option<GR>),
    Select(LR),
}

pub struct TabsRouter<GR, LR>
where
    GR: Switch + Debug,
    LR: Switch + Debug,
{
    _router: RouteAgentBridge,
    _marker: PhantomData<LR>,

    active: Option<GR>,
}

impl<GR, LR> Component for TabsRouter<GR, LR>
where
    GR: Switch + PartialEq + Clone + Debug + 'static,
    LR: Switch + PartialEq + Clone + Debug + 'static,
{
    type Message = TabsRouterMsg<GR, LR>;
    type Properties = Props<GR, LR>;

    fn create(ctx: &Context<Self>) -> Self {
        let callback = ctx.link().callback(|route: yew_router::route::Route| {
            TabsRouterMsg::RouteChange(Switch::switch(route))
        });
        let mut _router = RouteAgentBridge::new(callback);
        _router.send(RouteRequest::GetCurrentRoute);
        Self {
            active: None,
            _router,
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabsRouterMsg::RouteChange(route) => {
                self.active = route;
                true
            }
            TabsRouterMsg::Select(route) => {
                let route = (ctx.props().transformer.to)(route);
                RouteAgentDispatcher::<()>::new().send(RouteRequest::ChangeRoute(route.into()));
                false
            }
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-tabs");

        if ctx.props().r#box {
            classes.push("pf-m-box");
        }

        if ctx.props().vertical {
            classes.push("pf-m-vertical");
        }

        if ctx.props().filled {
            classes.push("pf-m-fill");
        }

        let local = self
            .active
            .as_ref()
            .and_then(|active| (ctx.props().transformer.from)(active));

        return html! {
            <div class={classes}>
                <ul class="pf-c-tabs__list">
                    { for ctx.props().children.iter().map(|mut c|{
                        let to = c.props.to.clone();
                        let props = Rc::make_mut(&mut c.props);
                        props.current = local.map(|local| *local == to.clone()).unwrap_or_default();
                        props.onselect = ctx.link().callback(move |_|TabsRouterMsg::Select(to.clone()));
                        c
                    }) }
                </ul>
            </div>
        };
    }
}

// tab router item

#[derive(Properties, Clone, Debug, PartialEq)]
pub struct TabRouterItemProps<R>
where
    R: Switch + PartialEq + Debug,
{
    /// The tab label
    pub label: String,
    /// The switch this item references to
    pub to: R,
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

pub struct TabRouterItem<R>
where
    R: Switch + PartialEq,
{
    _marker: PhantomData<R>,
}

impl<R> Component for TabRouterItem<R>
where
    R: Switch + PartialEq + Debug + 'static,
{
    type Message = TabRouterItemMsg;
    type Properties = TabRouterItemProps<R>;

    fn create(_: &Context<Self>) -> Self {
        Self {
            _marker: Default::default(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            TabRouterItemMsg::Clicked => {
                ctx.props().onselect.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-tabs__item");

        if ctx.props().current {
            classes.push("pf-m-current");
        }

        return html! {
            <li class={classes}>
                <button class="pf-c-tabs__link" onclick={ctx.link().callback(|_|TabRouterItemMsg::Clicked)}>
                    <span class="pf-c-tabs__item-text"> { &ctx.props().label } </span>
                </button>
            </li>
        };
    }
}
