//! Accordion
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;

use crate::prelude::wrap::wrapper_div_with_attributes;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionProperties {
    #[prop_or_default]
    pub bordered: bool,

    #[prop_or_default]
    pub large: bool,

    #[prop_or_default]
    pub children: ChildrenWithProps<AccordionItem>,
}

/// Accordion component
#[function_component(Accordion)]
pub fn accordion(props: &AccordionProperties) -> Html {
    let mut class = classes!("pf-v5-c-accordion");

    if props.bordered {
        class.extend(classes!("pf-m-bordered"));
    }

    if props.large {
        class.extend(classes!("pf-m-display-lg"));
    }

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct AccordionItemProperties {
    pub title: String,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub fixed: bool,

    #[prop_or_default]
    pub children: Children,

    /// Callback for clicking on the toggle
    #[prop_or_default]
    pub onclick: Callback<()>,
}

#[function_component(AccordionItem)]
pub fn accordion_item(props: &AccordionItemProperties) -> Html {
    let expanded = props.expanded;

    let mut content_class = classes!("pf-v5-c-accordion__expandable-content");
    let mut toggle_class = classes!("pf-v5-c-accordion__toggle");

    if props.fixed {
        content_class.extend(classes!("pf-m-fixed"));
    }

    if expanded {
        content_class.extend(classes!("pf-m-expanded"));
        toggle_class.extend(classes!("pf-m-expanded"));
    }

    html!(
        <>
            <h3>
                <button
                    class={toggle_class}
                    onclick={props.onclick.reform(|_|())}
                    type="button"
                    aria-expanded={expanded.to_string()}
                >
                    <span class="pf-v5-c-accordion__toggle-text">{ &props.title }</span>

                    <span class="pf-v5-c-accordion__toggle-icon">
                        <i class="fas fa-angle-right" aria-hidden="true"></i>
                    </span>
                </button>
            </h3>
            <div class={content_class} hidden={!expanded}>
                { for props.children.iter().map(|item| wrapper_div_with_attributes(item, &[("class", AttributeOrProperty::Static("pf-v5-c-accordion__expandable-content-body"))])) }
            </div>
        </>
    )
}
