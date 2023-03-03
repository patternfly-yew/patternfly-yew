//! Login page controls
mod footer;

pub use footer::*;

use crate::Title;
use yew::{prelude::*, virtual_dom::VChild};

/// Properties for [`Login`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginProperties {
    #[prop_or_default]
    pub header: Children,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub footer: Children,
}

/// Login page component
///
/// > A **login page** allows a user to gain access to an application by entering their username and password or by authenticating using a social media login.
///
/// See: <https://www.patternfly.org/v4/components/login-page>
///
/// ## Properties
///
/// Defined by [`LoginProperties`]
///
/// ## Children
///
/// The login page component requires a more complex structure using [`LoginMain`] components. See
/// the Quickstart project for an example.
#[function_component(Login)]
pub fn login(props: &LoginProperties) -> Html {
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

/// Properties for [`LoginMain`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMain)]
pub fn login_main(props: &LoginMainProperties) -> Html {
    html! {
        <main class="pf-c-login__main">
            { for props.children.iter() }
        </main>
    }
}

/// Properties for [`LoginMainHeader`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainHeaderProperties {
    pub title: VChild<Title>,
    #[prop_or_default]
    pub description: String,
}

#[function_component(LoginMainHeader)]
pub fn login_main_header(props: &LoginMainHeaderProperties) -> Html {
    html! {
        <header class="pf-c-login__main-header">
            { props.title.clone() }
            <p class="pf-c-login__main-header-desc">
                {&props.description}
            </p>
        </header>
    }
}

/// Properties for [`LoginMainBody`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainBodyProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(LoginMainBody)]
pub fn login_main_body(props: &LoginMainBodyProperties) -> Html {
    html! {
        <div class="pf-c-login__main-body">
            { for props.children.iter() }
        </div>
    }
}
