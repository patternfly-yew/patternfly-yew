use crate::prelude::Tooltip;
use yew::prelude::*;

/// Properties for [`LoginMainFooterLink`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterLinkProperties {
    #[prop_or_default]
    pub href: Option<String>,
    #[prop_or_default]
    pub onclick: Option<Callback<MouseEvent>>,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub target: Option<AttrValue>,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMainFooterLink)]
pub fn login_main_footer_link(props: &LoginMainFooterLinkProperties) -> Html {
    let link = html! (
        <a
            class="pf-v5-c-login__main-footer-links-item-link"
            href={props.href.clone()}
            onclick={props.onclick.clone()}
            target={props.target.clone()}
            aria_label={props.label.clone()}
            >
            { for props.children.iter() }
        </a>
    );

    if props.label.is_empty() {
        link
    } else {
        html! (
            <Tooltip text={props.label.clone()}>
                {link}
            </Tooltip>
        )
    }
}

/// Properties for [`LoginMainFooter`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub band: Children,
    #[prop_or_default]
    pub links: ChildrenWithProps<LoginMainFooterLink>,
}

#[function_component(LoginMainFooter)]
pub fn login_main_footer(props: &LoginMainFooterProperties) -> Html {
    html! (
        <footer class="pf-v5-c-login__main-footer">
            { for props.children.iter() }

            if !props.links.is_empty() {
                <ul class="pf-v5-c-login__main-footer-links">
                { for props.links.iter().map(|item|{
                    html!{ <li class="pf-v5-c-login__main-footer-links-item">{item}</li> }
                }) }
                </ul>
            }

            if !props.band.is_empty() {
                <div class="pf-v5-c-login__main-footer-band">
                { for props.band.iter().map(|item|{
                    html!{ <p class="pf-v5-c-login__main-footer-band-item">{item}</p> }
                }) }
                </div>
            }

        </footer>
    )
}
