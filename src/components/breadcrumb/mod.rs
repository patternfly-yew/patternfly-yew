//! Navigation breadcrumbs
use crate::utils::Ouia;
use yew::{html::ChildrenRenderer, prelude::*};

const OUIA: Ouia = Ouia::new("Breadcrumb");

#[cfg(feature = "router")]
mod router;
mod variant;

#[cfg(feature = "router")]
pub use router::*;

use variant::BreadcrumbItemVariant;

/// Properties for [`Breadcrumb`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<BreadcrumbItemVariant>,

    /// OUIA Component id
    #[prop_or_else(|| OUIA.generated_id())]
    pub ouia_id: String,

    /// OUIA Component Type
    #[prop_or_else(|| OUIA.component_type())]
    pub ouia_type: String,

    /// OUIA Component Safe
    #[prop_or(true)]
    pub ouia_safe: bool,
}

/// Breadcrumb component
///
/// > A **breadcrumb** provides page context to help users navigate more efficiently and understand where they are in the application hierarchy.
///
/// See: <https://www.patternfly.org/components/breadcrumb>
///
/// ## Properties
///
/// Defined by [`BreadcrumbProperties`].
///
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProperties) -> Html {
    let last = props.children.len() - 1;

    html!(
        <nav
            class="pf-v5-c-breadcrumb"
            aria-label={"breadcrumb"}
            data-ouia-component-id={props.ouia_id.clone()}
            data-ouia-component-type={props.ouia_type.clone()}
            data-ouia-safe={props.ouia_safe.to_string()}
        >
            <ol class="pf-v5-c-breadcrumb__list" role="list">
                {
                    for props.children.iter().enumerate().map(|(n,c)|item(c, n == last))
                }
            </ol>
        </nav>
    )
}

fn item(mut child: BreadcrumbItemVariant, last: bool) -> Html {
    child.set_current(last);

    html!(
        <li class="pf-v5-c-breadcrumb__item">
            <span class="pf-v5-c-breadcrumb__item-divider">
                <i class="fas fa-angle-right" aria-hidden="true"></i>
            </span>
            { child }
        </li>
    )
}

/// Properties for [`BreadcrumbItem`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbItemProperties {
    #[prop_or_default]
    pub href: AttrValue,
    #[prop_or_default]
    pub target: AttrValue,
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    current: bool,
}

#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProperties) -> Html {
    let mut class = Classes::from("pf-v5-c-breadcrumb__link");
    let mut aria_current = AttrValue::default();

    if props.current {
        class.push("pf-m-current");
        aria_current = AttrValue::from("page")
    }

    if props.href.is_empty() {
        props.children.clone()
    } else {
        html!(
            <a
                {class}
                href={&props.href}
                target={&props.target}
                aria-current={aria_current}
            >
                    { props.children.clone() }
            </a>
        )
    }
}
