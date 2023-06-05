//! Application launcher menu

use crate::{GlobalClose, Icon, ListDivider, Position};
use std::rc::Rc;
use yew::{
    html::ChildrenRenderer,
    prelude::*,
    virtual_dom::{VChild, VComp},
};

/// Properties for [`AppLauncher`]
#[derive(Clone, PartialEq, Properties)]
pub struct AppLauncherProperties {
    #[prop_or_default]
    pub toggle: Option<Html>,
    #[prop_or_default]
    pub children: ChildrenRenderer<AppLauncherChildVariant>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub position: Position,
}

#[doc(hidden)]
pub enum Msg {
    Toggle,
    Close,
}

/// Application launcher component
///
/// > An **application launcher** is an option menu that allows a user to launch a separate web application in a new browser window.
///
/// See: <https://www.patternfly.org/v4/components/application-launcher>
///
/// ## Properties
///
/// Defined by [`AppLauncherProperties`].
///
/// ## Children
///
/// Children of the application launcher are [`AppLauncherItem`]s. It is also possible use
/// [`crate::prelude::Divider`] to group entries.
pub struct AppLauncher {
    expanded: bool,
    global_close: GlobalClose,
}

impl Component for AppLauncher {
    type Message = Msg;
    type Properties = AppLauncherProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let global_close =
            GlobalClose::new(NodeRef::default(), ctx.link().callback(|_| Msg::Close));
        Self {
            expanded: false,
            global_close,
        }
    }

    fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::Toggle => {
                self.expanded = !self.expanded;
            }
            Msg::Close => self.expanded = false,
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v5-c-app-launcher");
        let mut menu_classes = Classes::from("pf-v5-c-app-launcher__menu");

        match ctx.props().position {
            Position::Left => {}
            Position::Right => menu_classes.push("pf-m-align-right"),
            Position::Top => classes.push("pf-m-top"),
        }

        if self.expanded {
            classes.push("pf-m-expanded");
        }

        let onclick = ctx.link().callback(|_| Msg::Toggle);

        html! {
            <nav
                class={classes}
                ref={self.global_close.clone()}
                >
                <button
                    class="pf-v5-c-app-launcher__toggle"
                    type="button"
                    disabled={ctx.props().disabled}
                    onclick={onclick}
                    >
                    { self.render_trigger(ctx.props()) }
                </button>
                <ul
                    class={menu_classes}
                    hidden={!self.expanded}
                    role="menu"
                >
                    { for ctx.props().children.iter().map(|mut c|{
                        // request a close callback from the item
                        c.set_need_close(ctx.link().callback(|_|Msg::Close));
                        c
                    }) }
                </ul>
            </nav>
        }
    }
}

impl AppLauncher {
    fn render_trigger(&self, props: &<AppLauncher as Component>::Properties) -> Html {
        if let Some(toggle) = &props.toggle {
            toggle.clone()
        } else {
            Icon::Th.into()
        }
    }
}

/// Child for an [`AppLauncher`]
#[derive(Clone, PartialEq)]
pub enum AppLauncherChild {
    Item(Rc<<AppLauncherItem as Component>::Properties>),
    Divider(Rc<<ListDivider as BaseComponent>::Properties>),
}

impl From<AppLauncherItemProperties> for AppLauncherChild {
    fn from(props: AppLauncherItemProperties) -> Self {
        AppLauncherChild::Item(Rc::new(props))
    }
}

impl From<()> for AppLauncherChild {
    fn from(_: ()) -> Self {
        AppLauncherChild::Divider(Rc::new(()))
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
                let props = Rc::make_mut(props);
                props.want_close = callback;
            }
            _ => {}
        }
    }
}

impl<CHILD> From<VChild<CHILD>> for AppLauncherChildVariant
where
    CHILD: BaseComponent,
    CHILD::Properties: Into<AppLauncherChild> + Clone,
{
    fn from(vchild: VChild<CHILD>) -> Self {
        Self {
            props: (*vchild.props).clone().into(),
        }
    }
}

impl Into<Html> for AppLauncherChildVariant {
    fn into(self) -> Html {
        match self.props {
            AppLauncherChild::Item(props) => VComp::new::<AppLauncherItem>(props, None).into(),
            AppLauncherChild::Divider(props) => VComp::new::<ListDivider>(props, None).into(),
        }
    }
}

// Item

/// Properties for [`AppLauncherItem`]
#[derive(Clone, PartialEq, Properties)]
pub struct AppLauncherItemProperties {
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

#[doc(hidden)]
#[derive(Copy, Clone)]
pub enum AppLauncherItemMsg {
    Clicked,
}

/// An item of an [`AppLauncher`] component.
///
/// ## Properties
///
/// Defined by [`AppLauncherItemProperties`].
pub struct AppLauncherItem {}

impl Component for AppLauncherItem {
    type Message = AppLauncherItemMsg;
    type Properties = AppLauncherItemProperties;

    fn create(_: &Context<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            AppLauncherItemMsg::Clicked => {
                if let Some(onclick) = &ctx.props().onclick {
                    onclick.emit(());
                }
                // request close from our parent
                ctx.props().want_close.emit(());
            }
        }
        false
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let action = if ctx.props().onclick.is_some() {
            html! (
                <button
                    class="pf-v5-c-app-launcher__menu-item"
                    onclick={ctx.link().callback(|_|Self::Message::Clicked)}
                    type="button"
                    role="menuitem"
                >
                    { for ctx.props().children.iter() }
                </button>
            )
        } else {
            let mut classes = Classes::from("pf-v5-c-app-launcher__menu-item");

            let target = if ctx.props().external {
                classes.push("pf-m-external");
                "_blank"
            } else {
                ""
            };

            html! (
                <a
                    class={classes}
                    target={target}
                    href={ctx.props().href.clone()}
                >

                { for ctx.props().children.iter() }

                if ctx.props().external {
                    <span class="pf-v5-c-app-launcher__menu-item-external-icon">
                        { Icon::ExternalLinkAlt }
                    </span>
                    <span class="pf-v5-u-screen-reader">{"(opens new window)"}</span>
                }

                </a>
            )
        };

        html! (
            <li>{action}</li>
        )
    }
}
