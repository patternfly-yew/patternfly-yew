use std::rc::Rc;
use yew::prelude::*;

use crate::prelude::OptionalHtml;

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ToggleGroupProperties {
    /// Content rendered inside the toggle group
    #[prop_or_default]
    pub children: ChildrenWithProps<ToggleGroupItem>,
    /// Additional classes added to the toggle group
    #[prop_or_default]
    pub class: Classes,
    /// Modifies the toggle group to include compact styling
    #[prop_or_default]
    pub compact: bool,
    /// Disable all toggle group items under this component
    #[prop_or_default]
    pub all_disabled: bool,
}

#[function_component(ToggleGroup)]
pub fn toggle_group(props: &ToggleGroupProperties) -> Html {
    let mut class = classes!(props.class.clone(), "pf-v6-c-toggle-group");
    if props.compact {
        class.push("pf-m-compact");
    };
    html! {
        <div {class} role="group">
            {for props.children.iter().map(|mut item| {
                let item_props = Rc::make_mut(&mut item.props);
                item_props.disabled |= props.all_disabled;
                item
            })}
        </div>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ToggleGroupItemProperties {
    /// Text rendered inside the toggle group item
    #[prop_or_default]
    pub text: OptionalHtml,
    /// Icon rendered inside the toggle group item
    #[prop_or_default]
    pub icon: OptionalHtml,
    /// Additional classes added to the toggle group item
    #[prop_or_default]
    pub class: Classes,
    /// Flag indicating if the toggle group item is disabled
    #[prop_or_default]
    pub disabled: bool,
    /// Flag indicating if the toggle group item is selected
    #[prop_or_default]
    pub selected: bool,
    /// A callback for when the toggle group item selection changes
    #[prop_or_default]
    pub onchange: Callback<()>,
    /// A reference to the button which emits the event for onchange.
    #[prop_or_default]
    pub button_ref: NodeRef,
}

#[function_component(ToggleGroupItem)]
pub fn toggle_group_item(props: &ToggleGroupItemProperties) -> Html {
    let mut class = classes!(props.class.clone(), "pf-v6-c-toggle-group__item");
    let mut button_class = classes!("pf-v6-c-toggle-group__button");
    if props.selected {
        class.push("pf-m-selected");
        button_class.push("pf-m-selected");
    }
    let onclick = use_callback(props.onchange.clone(), |_, onchange| onchange.emit(()));
    html! {
        <div {class}>
            <button
                type="button"
                class={button_class}
                {onclick}
                disabled={props.disabled}
                ref={props.button_ref.clone()}
            >
                if let Some(icon) = (*props.icon).as_ref() {
                    <ToggleGroupItemElement
                        variant={ToggleGroupItemElementVariant::Icon(icon.clone())}
                    />
                }
                if let Some(text) = (*props.text).as_ref() {
                    <ToggleGroupItemElement
                        variant={ToggleGroupItemElementVariant::Text(text.clone())}
                    />
                }
            </button>
        </div>
    }
}

#[derive(Debug, Clone, PartialEq)]
pub enum ToggleGroupItemElementVariant {
    Icon(Html),
    Text(Html),
}

#[derive(Debug, Clone, PartialEq, Properties)]
pub struct ToggleGroupItemElementProperties {
    #[prop_or_default]
    variant: Option<ToggleGroupItemElementVariant>,
}

#[function_component(ToggleGroupItemElement)]
pub fn toggle_group_item_element(props: &ToggleGroupItemElementProperties) -> Html {
    let (class, children) = props
        .variant
        .as_ref()
        .map(|v| match v {
            ToggleGroupItemElementVariant::Icon(children) => {
                ("pf-v6-c-toggle-group__icon", children)
            }
            ToggleGroupItemElementVariant::Text(children) => {
                ("pf-v6-c-toggle-group__text", children)
            }
        })
        .unzip();
    html! {
        <span {class} id="foo">
            if let Some(children) = children {
                {(*children).clone()}
            }
        </span>
    }
}
