use super::use_close_menu_callback;
use crate::hooks::id::use_prop_id;
use web_sys::HtmlInputElement;
use yew::prelude::*;

/// Properties for [`MenuCheckbox`].
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuCheckboxProperties {
    /// The label of the item
    #[prop_or_default]
    pub children: Html,

    /// Id of the checkbox input
    #[prop_or_default]
    pub id: Option<String>,

    /// The name of the checkbox input
    #[prop_or_default]
    pub name: Option<AttrValue>,

    #[prop_or_default]
    pub description: Option<String>,

    #[prop_or_default]
    pub disabled: bool,

    /// Flag if the checkbox is checked.
    #[prop_or_default]
    pub checked: bool,

    /// Callback when the state of the checkbox changes, providing the new state.
    #[prop_or_default]
    pub onchange: Callback<bool>,

    /// Close the surrounding menu (e.g. a dropdown) when the checkbox is toggled.
    #[prop_or_default]
    pub close_on_select: bool,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub class: Classes,
}

/// A menu item with a checkbox.
///
/// By default, toggling the checkbox does not close the surrounding menu. This can be changed
/// using the `close_on_select` property.
#[component]
pub fn MenuCheckbox(props: &MenuCheckboxProperties) -> Html {
    let id = use_prop_id(props.id.clone());
    let node_ref = use_node_ref();
    let onclose = use_close_menu_callback();

    let mut class = classes!("pf-v6-c-menu__list-item");
    if props.disabled {
        class.push(classes!("pf-m-disabled"));
    }
    class.extend(&props.class);

    let mut item_class = classes!("pf-v6-c-menu__item");
    if props.disabled {
        item_class.push(classes!("pf-m-disabled"));
    }

    let onchange = use_callback(
        (
            props.onchange.clone(),
            props.close_on_select,
            node_ref.clone(),
            onclose,
        ),
        |_: Event, (onchange, close_on_select, node_ref, onclose)| {
            if let Some(input) = node_ref.cast::<HtmlInputElement>() {
                onchange.emit(input.checked());
            }
            if *close_on_select {
                onclose.emit(());
            }
        },
    );

    html!(
        <li {class} style={&props.style}>
            <label class={item_class} for={(*id).clone()}>
                <span class="pf-v6-c-menu__item-main">
                    <span class="pf-v6-c-menu__item-check">
                        <span class="pf-v6-c-check pf-m-standalone">
                            <input
                                ref={node_ref}
                                class="pf-v6-c-check__input"
                                type="checkbox"
                                role="menuitemcheckbox"
                                id={(*id).clone()}
                                name={&props.name}
                                tabindex="-1"
                                checked={props.checked}
                                aria-checked={props.checked.to_string()}
                                disabled={props.disabled}
                                {onchange}
                            />
                        </span>
                    </span>
                    <span class="pf-v6-c-menu__item-text">{ props.children.clone() }</span>
                </span>
                if let Some(description) = &props.description {
                    <span class="pf-v6-c-menu__item-description"> {description} </span>
                }
            </label>
        </li>
    )
}
