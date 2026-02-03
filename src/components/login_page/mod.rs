//! Login page controls
mod footer;

pub use footer::*;

use crate::prelude::Title;
use yew::{prelude::*, virtual_dom::VChild};

/// Properties for [`Login`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginProperties {
    #[prop_or_default]
    pub header: Option<Html>,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub footer: Option<Html>,
}

/// Login page component
///
/// > A **login page** allows a user to gain access to an application by entering their username and password or by authenticating using a social media login.
///
/// See: <https://www.patternfly.org/components/login-page>
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
        <div class="pf-v6-c-login">
            <div class="pf-v6-c-login__container">
                if let Some(header) = &props.header {
                    <header class="pf-v6-c-login__header">{ header.clone() }</header>
                }
                { props.children.clone() }
                if let Some(footer) = &props.footer {
                    <footer class="pf-v6-c-login__footer">{ footer.clone() }</footer>
                }
            </div>
        </div>
    }
}

/// Properties for [`LoginMain`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(LoginMain)]
pub fn login_main(props: &LoginMainProperties) -> Html {
    let class = classes!("pf-v6-c-login__main", props.class.clone());
    html! {
        <main {class}>
            { props.children.clone() }
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
        <header class="pf-v6-c-login__main-header">
            { props.title.clone() }
            <p class="pf-v6-c-login__main-header-desc">
                {&props.description}
            </p>
        </header>
    }
}

/// Properties for [`LoginMainBody`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct LoginMainBodyProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(LoginMainBody)]
pub fn login_main_body(props: &LoginMainBodyProperties) -> Html {
    let class = classes!("pf-v6-c-login__main-body", props.class.clone());
    html! {
        <div {class}>
            { props.children.clone() }
        </div>
    }
}
