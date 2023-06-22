use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
struct MenuItemProperties {
    pub text: String,
    pub icon: Option<Html>,
    pub danger: bool,
    pub disabled: bool,
    pub r#type: MenuItemType,
}

#[derive(Clone, PartialEq, Debug)]
enum MenuItemType {
    Button(Callback<()>),
    Link { href: AttrValue, target: AttrValue },
}

#[function_component(MenuItem)]
fn menu_item(props: &MenuItemProperties) -> Html {
    let mut class = classes!("pf-v5-c-menu__list-item");

    if props.danger {
        class.push(classes!("pf-m-danger"));
    }

    if props.disabled {
        class.push(classes!("pf-m-disabled"));
    }

    let element = |content: Html| match &props.r#type {
        MenuItemType::Button(callback) => {
            html!(
                <button
                    class="pf-v5-c-menu__item"
                    type="button"
                    role="menuitem"
                    disabled={props.disabled}
                    onclick={callback.reform(|_|())}
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
            html!(
                <a
                    class="pf-v5-c-menu__item"
                    {href} {target}
                    aria-disabled={props.disabled.to_string()}
                    {tabindex}
                    role="menuitem"
                >
                    { content }
                </a>
            )
        }
    };

    html!(
        <div {class}>
            { element(html!(
                <span class="pf-v5-c-menu__item-main">
                    if let Some(icon) = &props.icon {
                        <span class="pf-v5-c-menu__item-icon"> {icon.clone()} </span>
                    }
                    if props.danger {
                        <span class="pf-v5-screen-reader">{ "Danger Item:" }</span>
                    }
                    <span class="pf-v5-c-menu__item-text">{ &props.text }</span>
                </span>
            )) }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuActionProperties {
    pub text: String,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub danger: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component(MenuAction)]
pub fn menu_action(props: &MenuActionProperties) -> Html {
    let MenuActionProperties {
        text,
        icon,
        danger,
        disabled,
        onclick,
    } = props.clone();
    html!(
        <MenuItem
            {text}
            {icon}
            {danger}
            {disabled}
            r#type={MenuItemType::Button(onclick)}
        />
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuLinkProperties {
    pub text: String,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub danger: bool,

    #[prop_or_default]
    pub disabled: bool,

    pub href: AttrValue,

    #[prop_or_default]
    pub target: AttrValue,
}

#[function_component(MenuLink)]
pub fn menu_link(props: &MenuLinkProperties) -> Html {
    let MenuLinkProperties {
        text,
        icon,
        danger,
        disabled,
        href,
        target,
    } = props.clone();
    html!(
        <MenuItem
            {text}
            {icon}
            {danger}
            {disabled}
            r#type={MenuItemType::Link{href, target}}
        />
    )
}
