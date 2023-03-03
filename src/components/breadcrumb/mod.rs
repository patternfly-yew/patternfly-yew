//! Navigation breadcrumbs
use yew::{html::ChildrenRenderer, prelude::*};

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
}

/// Breadcrumb component
///
/// > A **breadcrumb** provides page context to help users navigate more efficiently and understand where they are in the application hierarchy.
///
/// See: <https://www.patternfly.org/v4/components/breadcrumb>
///
/// ## Properties
///
/// Defined by [`BreadcrumbProperties`].
///
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProperties) -> Html {
    let last = props.children.len() - 1;

    html!(
        <nav class="pf-c-breadcrumb" aria-label={"breadcrumb"}>
            <ol class="pf-c-breadcrumb__list">
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
        <li class="pf-c-breadcrumb__item">
            <span class="pf-c-breadcrumb__item-divider">
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
    pub children: Children,
    #[prop_or_default]
    current: bool,
}

#[function_component(BreadcrumbItem)]
pub fn breadcrumb_item(props: &BreadcrumbItemProperties) -> Html {
    let mut class = Classes::from("pf-c-breadcrumb__link");

    if props.current {
        class.push("pf-m-current");
    }

    if props.href.is_empty() {
        html!(
            { for props.children.iter() }
        )
    } else {
        html!(
            <a
                {class}
                href={&props.href}
                target={&props.target}
            >
                    { for props.children.iter() }
            </a>
        )
    }
}
