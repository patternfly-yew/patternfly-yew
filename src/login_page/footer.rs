use crate::Tooltip;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterLinkProps {
    #[prop_or_default]
    pub href: String,
    #[prop_or_default]
    pub label: String,
    #[prop_or_default]
    pub target: String,
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMainFooterLink)]
pub fn login_main_footer_link(props: &LoginMainFooterLinkProps) -> Html {
    let link = html! {
        <a
            class="pf-c-login__main-footer-links-item-link"
            href={props.href.clone()}
            target={props.target.clone()}
            aria_label={props.label.clone()}
            >
            { for props.children.iter() }
        </a>
    };

    if props.label.is_empty() {
        link
    } else {
        html! {
            <Tooltip text={props.label.clone()}>
                {link}
            </Tooltip>
        }
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainFooterProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub band: Children,
    #[prop_or_default]
    pub links: ChildrenWithProps<LoginMainFooterLink>,
}

#[function_component(LoginMainFooter)]
pub fn login_main_footer(props: &LoginMainFooterProps) -> Html {
    html! {
        <footer class="pf-c-login__main-footer">
            { for props.children.iter() }

            { if props.links.len() > 0 {
                html!{
                    <ul class="pf-c-login__main-footer-links">
                    { for props.links.iter().map(|item|{
                        html!{ <li class="pf-c-login__main-footer-links-item">{item}</li> }
                    }) }
                    </ul>
                }
            } else {
                html!{}
            }}

            { if props.band.len() > 0 {
                html!{
                    <div class="pf-c-login__main-footer-band">
                    { for props.band.iter().map(|item|{
                        html!{ <p class="pf-c-login__main-footer-band-item">{item}</p> }
                    }) }
                    </div>
                }
            } else {
                html!{}
            }}

        </footer>
    }
}
