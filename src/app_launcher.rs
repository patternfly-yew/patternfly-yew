use crate::{Button, Divider, GlobalClose, Icon, Position};
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub toggle: Option<Html>,
    #[prop_or_default]
    pub children: ChildrenRenderer<AppLauncherChildVariant>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub position: Position,
}

pub enum Msg {
    Toggle,
    Close,
}

pub struct AppLauncher {
    props: Props,
    link: ComponentLink<Self>,

    expanded: bool,
    global_close: GlobalClose,
}

impl Component for AppLauncher {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let global_close = GlobalClose::new(NodeRef::default(), link.callback(|_| Msg::Close));
        Self {
            expanded: false,
            props,
            link,
            global_close,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
            Msg::Close => self.expanded = false,
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
        let mut classes = Classes::from("pf-c-app-launcher");
        let mut menu_classes = Classes::from("pf-c-app-launcher__menu");

        match self.props.position {
            Position::Left => {}
            Position::Right => menu_classes.push("pf-m-align-right"),
            Position::Top => classes.push("pf-m-top"),
        }

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        let onclick = self.link.callback(|_| Msg::Toggle);

        return html! {
            <nav
                class=classes
                ref=self.global_close.clone()
                >
                <Button
                    class="pf-c-app-launcher__toggle"
                    r#type="button"
                    disabled=self.props.disabled
                    onclick=onclick
                    >
                    { self.render_trigger() }
                </Button>
                <ul
                    class=menu_classes
                    hidden=!self.expanded
                >
                    { for self.props.children.iter().map(|mut c|{
                        // request a close callback from the item
                        c.set_need_close(self.link.callback(|_|Msg::Close));
                        c
                    }) }
                </ul>
            </nav>
        };
    }
}

impl AppLauncher {
    fn render_trigger(&self) -> Html {
        if let Some(toggle) = &self.props.toggle {
            toggle.clone()
        } else {
            Icon::Th.into()
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum AppLauncherChild {
    Item(<AppLauncherItem as Component>::Properties),
    Divider(<Divider as Component>::Properties),
}

impl From<AppLauncherItemProps> for AppLauncherChild {
    fn from(props: AppLauncherItemProps) -> Self {
        AppLauncherChild::Item(props)
    }
}

impl From<()> for AppLauncherChild {
    fn from(_: ()) -> Self {
        AppLauncherChild::Divider(())
    }
}

#[derive(PartialEq, Clone)]
pub struct AppLauncherChildVariant {
    props: AppLauncherChild,
}

impl AppLauncherChildVariant {
    /// Forward the need to get a close callback to the actual item
    fn set_need_close(&mut self, callback: Callback<()>) {
        match self.props {
            AppLauncherChild::Item(ref mut props) => {
                props.want_close = callback;
            }
            _ => {}
        }
    }
}

impl<CHILD> From<VChild<CHILD>> for AppLauncherChildVariant
where
    CHILD: Component,
    CHILD::Properties: Into<AppLauncherChild>,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: vchild.props.into(),
        }
    }
}

impl Into<Html> for AppLauncherChildVariant {
    fn into(self) -> Html {
        match self.props {
            AppLauncherChild::Item(props) => {
                VComp::new::<AppLauncherItem>(props, NodeRef::default(), None).into()
            }
            AppLauncherChild::Divider(props) => {
                VComp::new::<Divider>(props, NodeRef::default(), None).into()
            }
        }
    }
}

// Item

#[derive(Clone, PartialEq, Properties)]
pub struct AppLauncherItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
    #[prop_or_default]
    pub(crate) want_close: Callback<()>,
    #[prop_or_default]
    pub external: bool,
}

#[derive(Copy, Clone)]
pub enum AppLauncherItemMsg {
    Clicked,
}

#[derive(Clone)]
pub struct AppLauncherItem {
    props: AppLauncherItemProps,
    link: ComponentLink<Self>,
}

impl Component for AppLauncherItem {
    type Message = AppLauncherItemMsg;
    type Properties = AppLauncherItemProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { props, link }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            AppLauncherItemMsg::Clicked => {
                if let Some(onclick) = &self.props.onclick {
                    onclick.emit(());
                }
                // request close from our parent
                self.props.want_close.emit(());
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
        let action = if self.props.onclick.is_some() {
            html! {
                <Button
                    class="pf-c-app-launcher__menu-item"
                    onclick=self.link.callback(|_|Self::Message::Clicked)
                    >
                    { for self.props.children.iter() }
                </Button>
            }
        } else {
            let mut classes = Classes::from("pf-c-app-launcher__menu-item");

            let target = if self.props.external {
                classes.push("pf-m-external");
                "_blank"
            } else {
                ""
            };

            html! {
                <a
                    class=classes
                    target=target
                    href=self.props.href.clone()>

                { for self.props.children.iter() }

                { if self.props.external {html!{
                    <>
                    <span class="pf-c-app-launcher__menu-item-external-icon">
                        { Icon::ExternalLinkAlt }
                    </span>
                    <span class="pf-screen-reader">{"(opens new window)"}</span>
                    </>
                }} else {html!{}} }

                </a>
            }
        };

        return html! {
            <li>{action}</li>
        };
    }
}
