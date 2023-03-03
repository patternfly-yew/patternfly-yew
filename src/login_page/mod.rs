mod footer;

pub use footer::*;

use crate::Title;
use yew::{prelude::*, virtual_dom::VChild};

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub header: Children,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub footer: Children,
}

#[function_component(Login)]
pub fn login(props: &Props) -> Html {
    html! {
        <div class="pf-c-login">
            <div class="pf-c-login__container">
                if !props.header.is_empty() {
                    <header class="pf-c-login__header">{for props.header.iter()}</header>
                }
                { for props.children.iter() }
                if !props.footer.is_empty() {
                    <footer class="pf-c-login__footer">{for props.footer.iter()}</footer>
                }
            </div>
        </div>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMain)]
pub fn login_main(props: &LoginMainProps) -> Html {
    html! {
        <main class="pf-c-login__main">
            { for props.children.iter() }
        </main>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainHeaderProps {
    pub title: VChild<Title>,
    #[prop_or_default]
    pub description: String,
}

#[function_component(LoginMainHeader)]
pub fn login_main_header(props: &LoginMainHeaderProps) -> Html {
    html! {
        <header class="pf-c-login__main-header">
            { props.title.clone() }
            <p class="pf-c-login__main-header-desc">
                {&props.description}
            </p>
        </header>
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainBodyProps {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMainBody)]
pub fn login_main_body(props: &LoginMainBodyProps) -> Html {
    html! {
        <div class="pf-c-login__main-body">
            { for props.children.iter() }
        </div>
    }
}
