//! Full Page components
use crate::prelude::{Button, ButtonType, ButtonVariant};
use std::rc::Rc;
use yew::prelude::*;

mod section;
mod sidebar;

pub use section::*;
pub use sidebar::*;

/// Properties for [`Page`]
#[derive(Clone, PartialEq, Properties)]
pub struct PageProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub sidebar: ChildrenWithProps<PageSidebar>,
    #[prop_or_default]
    pub tools: Html,

    /// The brand section.
    ///
    /// Expected to be a single [`MastheadBrand`] component.
    ///
    /// NOTE: Future versions might enforce the child requirement without prior deprecation.
    #[prop_or_default]
    pub brand: Html,

    #[prop_or_default]
    pub nav: Html,
    #[prop_or(true)]
    pub open: bool,
    #[prop_or_default]
    pub full_height: bool,

    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// A full page
///
/// > The page component is used to define the basic layout of a page with either vertical or horizontal navigation.
///
/// See: <https://www.patternfly.org/components/page>
///
/// ## Properties
///
/// Defined by [`PageProperties`].
///
/// ## Elements
///
/// * **Sidebar**: Contains a single [`PageSidebar`], hosting the main navigation.
/// * **Navigation**: The top header navigation section.
/// * **Tools**: Tools, shown in the header section of the page.
/// * **Brand**: A brand logo, shown in the navigation header section.
/// * **Children**: The actual page content, probably wrapped into [`PageSection`] components.
///
#[function_component(Page)]
pub fn page(props: &PageProperties) -> Html {
    let open = use_state_eq(|| props.open);

    let onclick = {
        let open = open.clone();
        Callback::from(move |_| {
            open.set(!(*open));
        })
    };

    html! (
        <div class="pf-v5-c-page" id={&props.id} role="main" tabindex="-1">
            <header class="pf-v5-c-masthead">
                // If the sidebar is empty then the toggle button is not
                // shown
                if !props.sidebar.is_empty() {
                    <span class="pf-v5-c-masthead__toggle">
                        <Button
                            r#type={ButtonType::Button}
                            variant={ButtonVariant::Plain}
                            {onclick}
                        >
                            <i class="fas fa-bars" aria-hidden="true" />
                        </Button>
                    </span>
                }

                <div class="pf-v5-c-masthead__main">
                    { props.brand.clone() }
                </div>

                <div class="pf-v5-c-masthead__content"> // TODO: Should migrate props
                    { props.nav.clone() }
                    { props.tools.clone() }
                </div>

            </header>

            { for props.sidebar.iter().map(|mut s|{
                let props = Rc::make_mut(&mut s.props);
                props.open = *open;
                s
            }) }

            <main class="pf-v5-c-page__main" tabindex="-1">
                { props.children.clone() }
            </main>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MastheadBrandProperties {
    /// Expected to be a single [`crate::prelude::Brand`] component.
    ///
    /// NOTE: Future versions might enforce the child requirement without prior deprecation.
    #[prop_or_default]
    pub children: Html,

    /// Called when the user clicks on the brand logo.
    #[prop_or_default]
    pub onclick: Option<Callback<()>>,
}

/// Masthead brand component.
///
/// ## Properties
///
/// Defined by [`MastheadBrandProperties`].
///
/// ## Children
///
/// A single [`crate::prelude::Brand`] component. The children may be wrapped in an `a` element when the `onclick`
/// callback is set.
#[function_component(MastheadBrand)]
pub fn masthead_brand(props: &MastheadBrandProperties) -> Html {
    match &props.onclick {
        Some(onclick) => {
            let onclick = onclick.reform(|_| ());
            html!(
                <a class="pf-v5-c-masthead__brand" href="#" {onclick}>
                    { props.children.clone() }
                </a>
            )
        }
        None => {
            html!(
                <div class="pf-v5-c-masthead__brand">
                    { props.children.clone() }
                </div>
            )
        }
    }
}
