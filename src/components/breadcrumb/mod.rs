//! Navigation breadcrumbs
use crate::ouia;
use crate::prelude::OuiaComponentType;
use crate::utils::{Ouia, OuiaSafe};
use variant::BreadcrumbItemVariant;
use yew::{html::ChildrenRenderer, prelude::*};

#[cfg(feature = "yew-nested-router")]
mod router;
mod variant;

#[cfg(feature = "yew-nested-router")]
pub use router::*;

const OUIA: Ouia = ouia!("Breadcrumb");

/// Properties for [`Breadcrumb`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct BreadcrumbProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<BreadcrumbItemVariant>,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
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
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let last = props.children.len() - 1;

    html!(
        <nav
            class="pf-v6-c-breadcrumb"
            aria-label={"breadcrumb"}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            <ol class="pf-v6-c-breadcrumb__list" role="list">
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
        <li class="pf-v6-c-breadcrumb__item">
            <span class="pf-v6-c-breadcrumb__item-divider">
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
    let mut class = Classes::from("pf-v6-c-breadcrumb__link");
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
