//! Navigation controls
#[cfg(feature = "yew-nested-router")]
mod router;

#[cfg(feature = "yew-nested-router")]
pub use router::*;
use std::collections::HashSet;

use crate::ouia;
use crate::prelude::{Icon, Id, OuiaComponentType};
use crate::utils::{Ouia, OuiaSafe};
use std::fmt::Debug;
use yew::prelude::*;

const OUIA_NAV: Ouia = ouia!("Nav");
const OUIA_NAV_ITEM: Ouia = ouia!("NavItem");

// nav

/// Properties for [`Nav`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct NavProperties {
    #[prop_or_default]
    pub children: Html,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA_NAV.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

/// A navigation component.
#[function_component(Nav)]
pub fn nav(props: &NavProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA_NAV.generated_id())
    });
    html! {
        <nav
            class="pf-v6-c-nav"
            aria-label="Global"
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            { props.children.clone() }
        </nav>
    }
}

// nav list

/// Properties for [`NavList`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavListProperties {
    #[prop_or_default]
    pub children: Html,
}

#[function_component(NavList)]
pub fn nav_list(props: &NavListProperties) -> Html {
    html! {
        <ul class="pf-v6-c-nav__list" role="list">
            { props.children.clone() }
        </ul>
    }
}

// nav group

/// Properties for [`NavGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavGroupProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub title: String,
}

#[function_component(NavGroup)]
pub fn nav_group(props: &NavGroupProperties) -> Html {
    html! {
        <section class="pf-v6-c-nav__section">
            <h2 class="pf-v6-c-nav__section-title">{ props.title.clone() }</h2>
            <NavList>
                { props.children.clone() }
            </NavList>
        </section>
    }
}

// nav item

/// Properties for [`NavItem`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavItemProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub onclick: Callback<()>,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA_NAV_ITEM.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
}

/// A navigation item, which triggers a callback when clicked.
#[function_component(NavItem)]
pub fn nav_item(props: &NavItemProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA_NAV_ITEM.generated_id())
    });
    html! (
        <li
            class="pf-v6-c-nav__item"
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <a
                href="#"
                class="pf-v6-c-nav__link"
                onclick={props.onclick.reform(|_|())}
            >
                { props.children.clone() }
            </a>
        </li>
    )
}

/// Properties for [`NavItem`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavLinkProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub href: AttrValue,
    #[prop_or_default]
    pub target: Option<AttrValue>,
}

/// A navigation item, which is a link.
#[function_component(NavLink)]
pub fn nav_link(props: &NavLinkProperties) -> Html {
    html! (
        <li class="pf-v6-c-nav__item">
            <a
                href={&props.href}
                class="pf-v6-c-nav__link"
                target={&props.target}
            >
                { props.children.clone() }
            </a>
        </li>
    )
}

#[derive(Clone, PartialEq)]
pub struct Expandable {
    callback: Callback<(Id, bool)>,
}

impl Expandable {
    pub fn state(&self, id: Id, active: bool) {
        self.callback.emit((id, active));
    }
}

// nav expandable

/// Properties for [`NavExpandable`]
#[derive(Clone, PartialEq, Properties)]
pub struct NavExpandableProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub title: String,
    #[prop_or_default]
    pub expanded: bool,
}

/// Expandable navigation group/section.
pub struct NavExpandable {
    expanded: Option<bool>,
    context: Expandable,
    active: HashSet<Id>,
}

#[doc(hidden)]
#[derive(Clone, Debug)]
pub enum MsgExpandable {
    Toggle,
    ChildState(Id, bool),
}

impl Component for NavExpandable {
    type Message = MsgExpandable;
    type Properties = NavExpandableProperties;

    fn create(ctx: &Context<Self>) -> Self {
        let expanded = match ctx.props().expanded {
            true => Some(true),
            false => None,
        };

        log::debug!("Creating new NavExpandable");

        let callback = ctx
            .link()
            .callback(|(id, state)| MsgExpandable::ChildState(id, state));

        Self {
            expanded,
            active: Default::default(),
            context: Expandable { callback },
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgExpandable::Toggle => {
                self.expanded = Some(!self.is_expanded(ctx));
            }
            MsgExpandable::ChildState(id, state) => match state {
                true => {
                    self.active.insert(id);
                }
                false => {
                    self.active.remove(&id);
                }
            },
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>, _: &Self::Properties) -> bool {
        if ctx.props().expanded {
            self.expanded = Some(true);
        }
        true
    }

    fn rendered(&mut self, ctx: &Context<Self>, first_render: bool) {
        if first_render && self.expanded.is_none() && self.is_expanded(ctx) {
            // if this is the first render, and we are expanded, we want to stay that way.
            // Unless a user explicitly toggles.
            self.expanded = Some(true);
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-v6-c-nav__item pf-m-expandable");

        let expanded = self.is_expanded(ctx);

        if expanded {
            classes.push("pf-m-expanded");
        }

        let context = self.context.clone();

        html! {
            <ContextProvider<Expandable> {context}>
                <li class={classes}>
                    <button
                        class="pf-v6-c-nav__link"
                        aria-expanded={expanded.to_string()}
                        onclick={ctx.link().callback(|_|MsgExpandable::Toggle)}
                    >
                        { &ctx.props().title }
                        <span class="pf-v6-c-nav__toggle">
                            <span class="pf-v6-c-nav__toggle-icon">
                                { Icon::AngleRight }
                            </span>
                        </span>
                    </button>

                    <section class="pf-v6-c-nav__subnav" hidden={!expanded}>
                        <NavList>
                            { ctx.props().children.clone() }
                        </NavList>
                    </section>
                </li>
            </ContextProvider<Expandable>>
        }
    }
}

impl NavExpandable {
    fn is_expanded(&self, ctx: &Context<Self>) -> bool {
        // if we have a current state, that will always override.
        self.expanded.unwrap_or_else(|| {
            // if any child is currently active.
            let active = !self.active.is_empty();

            ctx.props().expanded || active
        })
    }
}

/// Access a wrapping [`Expandable`] content.
#[hook]
pub fn use_expandable() -> Option<Expandable> {
    use_context::<Expandable>()
}
