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
#[function_component(Nav)]
pub fn nav(props: &NavProps) -> Html {
    html! {
        <nav class="pf-c-nav" aria-label="Global">
            { for props.children.iter() }
        </nav>
    }
}

// nav list

#[derive(Clone, PartialEq, Properties)]
pub struct NavListProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(NavList)]
pub fn nav_list(props: &NavListProps) -> Html {
    html! {
        <ul class="pf-c-nav__list">
            { for props.children.iter() }
        </ul>
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

#[function_component(NavGroup)]
pub fn nav_group(props: &NavGroupProps) -> Html {
    html! {
        <section class="pf-c-nav__section">
            <h2 class="pf-c-nav__section-title">{ props.title.clone() }</h2>
            <NavList>
                { for props.children.iter() }
            </NavList>
        </section>
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

#[function_component(NavItem)]
pub fn nav_item(props: &NavItemProps) -> Html {
    let mut target = props.target.to_string();
    if target.is_empty() && props.external {
        target = "_blank".to_string();
    }

    let href = if props.to.is_empty() {
        "#".into()
    } else {
        props.to.clone()
    };

    return html! {
        <li class="pf-c-nav__item">
            <a
                href={href}
                class="pf-c-nav__link"
                target={target}
            >
                { for props.children.iter() }
                if props.external {
                    <span class="pf-u-ml-sm pf-u-font-size-sm">{Icon::ExternalLinkAlt}</span>
                }
            </a>
        </li>
    };
}

// nav expandable

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
    expanded: Option<bool>,
}

#[derive(Clone, Debug)]
pub enum MsgExpandable {
    Toggle,
}

impl Component for NavExpandable {
    type Message = MsgExpandable;
    type Properties = NavExpandableProps;

    fn create(ctx: &Context<Self>) -> Self {
        let expanded = match ctx.props().expanded {
            true => Some(true),
            false => None,
        };

        Self { expanded }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            MsgExpandable::Toggle => {
                self.expanded = Some(!self.is_expanded(ctx));
            }
        }
        true
    }

    fn changed(&mut self, ctx: &Context<Self>) -> bool {
        if ctx.props().expanded {
            self.expanded = Some(true);
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let mut classes = Classes::from("pf-c-nav__item pf-c-expandable");

        let expanded = self.is_expanded(ctx);

        if expanded {
            classes.push("pf-m-expanded");
        }

        return html! {
            <li class={classes}>
                <button
                    class="pf-c-nav__link"
                    aria-expanded={expanded.to_string()}
                    onclick={ctx.link().callback(|_|MsgExpandable::Toggle)}
                    >
                    { &ctx.props().title }
                    <span class="pf-c-nav__toggle">
                        <span class="pf-c-nav__toggle-icon">
                            { Icon::AngleRight }
                        </span>
                    </span>
                </button>

                <section class="pf-c-nav__subnav" hidden={!expanded}>
                    <NavList>
                        { for ctx.props().children.iter() }
                    </NavList>
                </section>
            </li>
        };
    }
}

impl NavExpandable {
    fn is_expanded(&self, ctx: &Context<Self>) -> bool {
        self.expanded.unwrap_or(ctx.props().expanded)
    }
}
