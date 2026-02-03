//! Accordion
use crate::prelude::wrap::wrapper_div_with_attributes;
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;

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
    let mut class = classes!("pf-v6-c-accordion");

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
    let expanded = use_state(|| props.expanded);

    use_effect_with(
        (props.expanded, expanded.setter()),
        |(expanded, expanded_setter)| expanded_setter.set(*expanded),
    );

    let onclick = use_callback(
        (expanded.clone(), props.onclick.clone()),
        |_, (expanded, onclick)| {
            expanded.set(!**expanded);
            onclick.emit(());
        },
    );

    let mut item_class = classes!("pf-v6-c-accordion__item");
    let mut content_class = classes!("pf-v6-c-accordion__expandable-content");
    let mut toggle_class = classes!("pf-v6-c-accordion__toggle");

    if props.fixed {
        content_class.extend(classes!("pf-m-fixed"));
    }

    if *expanded {
        item_class.extend(classes!("pf-m-expanded"));
        toggle_class.extend(classes!("pf-m-expanded"));
    }

    html!(
        <div class={item_class}>
            <dt>
                <button
                    class={toggle_class}
                    {onclick}
                    type="button"
                    aria-expanded={(*expanded).to_string()}
                >
                    <span class="pf-v6-c-accordion__toggle-text">{ &props.title }</span>

                    <span class="pf-v6-c-accordion__toggle-icon">
                        <i class="fas fa-angle-right" aria-hidden="true"></i>
                    </span>
                </button>
            </dt>
            <dd class={content_class} hidden={!(*expanded)}>
                { for props.children.iter().map(|item| wrapper_div_with_attributes(item, &[("class", AttributeOrProperty::Static("pf-v6-c-accordion__expandable-content-body"))])) }
            </dd>
        </div>
    )
}
