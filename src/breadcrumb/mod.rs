use yew::html::ChildrenRenderer;
use yew::prelude::*;

#[cfg(feature = "router")]
mod router;
mod variant;

#[cfg(feature = "router")]
pub use router::*;

use crate::breadcrumb::variant::BreadcrumbItemVariant;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbProps {
    #[prop_or_default]
    pub children: ChildrenRenderer<BreadcrumbItemVariant>,
}

/// Breadcrumb component
///
/// > A **breadcrumb** provides page context to help users navigate more efficiently and understand where they are in the application hierarchy.
///
/// See: https://www.patternfly.org/v4/components/breadcrumb
///
/// ## Properties
///
/// Defined by [`BreadcrumbProps`].
///
#[function_component(Breadcrumb)]
pub fn breadcrumb(props: &BreadcrumbProps) -> Html {
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

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbItemProps {
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
pub fn breadcrumb_item(props: &BreadcrumbItemProps) -> Html {
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
