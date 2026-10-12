use super::use_close_menu_callback;
use crate::prelude::Icon;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
struct MenuItemProperties {
    pub children: Html,
    pub icon: Option<Html>,
    pub danger: bool,
    pub disabled: bool,
    pub selected: bool,
    pub r#type: MenuItemType,
    pub close_on_select: bool,
    pub description: Option<String>,
    pub style: Option<AttrValue>,
    pub class: Classes,
}

#[derive(Clone, PartialEq, Debug)]
enum MenuItemType {
    Button(Callback<()>),
    Link { href: AttrValue, target: AttrValue },
}

#[component]
fn MenuItem(props: &MenuItemProperties) -> Html {
    let mut class = classes!("pf-v6-c-menu__list-item");

    if props.danger {
        class.push(classes!("pf-m-danger"));
    }

    if props.disabled {
        class.push(classes!("pf-m-disabled"));
    }

    let onclose = use_close_menu_callback();

    let mut item_class = classes!("pf-v6-c-menu__item");
    if props.selected {
        item_class.push(classes!("pf-m-selected"));
    }

    let element = |content: Html| match &props.r#type {
        MenuItemType::Button(callback) => {
            let callback = callback.clone();
            let close_on_select = props.close_on_select;

            html!(
                <button
                    class={item_class}
                    type="button"
                    role="menuitem"
                    tabindex="-1"
                    disabled={props.disabled}
                    onclick={callback.reform(move |_| {
                        if close_on_select {
                            onclose.emit(());
                        }
                    })}
                >
                    { content }
                </button>
            )
        }
        MenuItemType::Link { href, target } => {
            let tabindex = match props.disabled {
                true => Some("-1"),
                false => None,
            };

            let onclick = props.close_on_select.then(|| onclose.reform(|_| ()));

            html!(
                <a
                    class={item_class}
                    {href}
                    {target}
                    {onclick}
                    aria-disabled={props.disabled.to_string()}
                    {tabindex}
                    role="menuitem"
                >
                    { content }
                </a>
            )
        }
    };

    class.extend(&props.class);

    html!(
        <li {class} style={&props.style}>
            { element(html!(
                <>
                    <span class="pf-v6-c-menu__item-main">
                        if let Some(icon) = &props.icon {
                            <span class="pf-v6-c-menu__item-icon"> {icon.clone()} </span>
                        }
                        if props.danger {
                            <span class="pf-v6-screen-reader">{ "Danger Item:" }</span>
                        }

                        <span class="pf-v6-c-menu__item-text">{ props.children.clone() }</span>

                        if props.selected {
                            <span class="pf-v6-c-menu__item-select-icon">{ Icon::Check }</span>
                        }
                    </span>
                    if let Some(description) = &props.description {
                        <span class="pf-v6-c-menu__item-description"> {description} </span>
                    }
                </>
            )) }
        </li>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuActionProperties {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub danger: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub onclick: Callback<()>,

    #[prop_or_default]
    pub selected: bool,

    /// Close the surrounding menu (e.g. a dropdown) when the item is selected.
    #[prop_or(true)]
    pub close_on_select: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

#[component]
pub fn MenuAction(props: &MenuActionProperties) -> Html {
    // we use destructing and struct initialization here to ensure we're not missing any new field

    let MenuActionProperties {
        children,
        icon,
        danger,
        disabled,
        onclick,
        description,
        selected,
        close_on_select,
        style,
        class,
    } = props.clone();

    let props = MenuItemProperties {
        children,
        icon,
        danger,
        disabled,
        r#type: MenuItemType::Button(onclick),
        close_on_select,
        description,
        selected,
        style,
        class,
    };

    html!(<MenuItem ..props />)
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuLinkProperties {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub danger: bool,

    #[prop_or_default]
    pub disabled: bool,

    pub href: AttrValue,

    #[prop_or_default]
    pub target: AttrValue,

    #[prop_or_default]
    pub selected: bool,

    /// Close the surrounding menu (e.g. a dropdown) when the item is selected.
    #[prop_or(true)]
    pub close_on_select: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

#[component()]
pub fn MenuLink(props: &MenuLinkProperties) -> Html {
    // we use destructing and struct initialization here to ensure we're not missing any new field

    let MenuLinkProperties {
        children,
        icon,
        danger,
        disabled,
        href,
        target,
        description,
        selected,
        close_on_select,
        style,
        class,
    } = props.clone();

    let props = MenuItemProperties {
        children,
        icon,
        danger,
        disabled,
        r#type: MenuItemType::Link { href, target },
        close_on_select,
        description,
        selected,
        style,
        class,
    };

    html!(<MenuItem ..props />)
}
