use crate::prelude::ButtonType;
use yew::prelude::*;

/// Component for selecting an item out of a list.
/// Does not support grouping.
/// If you are looking for a grouped version then use
/// [`SimpleListGrouped`].
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SimpleListProperties {
    /// The items from which can be selected.
    #[prop_or_default]
    pub children: ChildrenWithProps<SimpleListItem>,
    /// Additional classes to add to the list.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SimpleList)]
pub fn simple_list(props: &SimpleListProperties) -> Html {
    html! {
        <SimpleListInner class={props.class.clone()}>
            <ul class={"pf-v6-c-simple-list__list"} role="list">
                { for props.children.iter() }
            </ul>
        </SimpleListInner>
    }
}

/// Component for selecting an item out of groups of lists.
/// If you are looking for a non-grouped version then use
/// [`SimpleList`].
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SimpleListGroupedProperties {
    /// The groups (which contain their items) from which can be selected.
    #[prop_or_default]
    pub children: ChildrenWithProps<SimpleListGroup>,
    /// Additional classes to add to the list.
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(SimpleListGrouped)]
pub fn simple_list_grouped(props: &SimpleListGroupedProperties) -> Html {
    html! {
        <SimpleListInner class={props.class.clone()}>
            {for props.children.iter()}
        </SimpleListInner>
    }
}

#[derive(Debug, Clone, PartialEq, Properties)]
struct SimpleListInnerProperties {
    #[prop_or_default]
    children: Html,
    #[prop_or_default]
    class: Classes,
}

#[function_component(SimpleListInner)]
fn simple_list_inner(props: &SimpleListInnerProperties) -> Html {
    let class = classes!("pf-v6-c-simple-list", props.class.clone());
    html! {
        <div {class}>{props.children.clone()}</div>
    }
}

/// A single item which can be selected in a [`SimpleList`].
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SimpleListItemProperties {
    /// The content to be rendered inside the item.
    #[prop_or_default]
    pub children: Html,
    /// Additional classes to pass to the item.
    #[prop_or_default]
    pub class: Classes,
    /// Additional classes to pass to the underlying button.
    #[prop_or_default]
    pub button_class: Classes,
    /// Whether the item is currently selected or not.
    #[prop_or_default]
    pub active: bool,
    /// Callback that is triggered when this item is clicked.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    /// Type of the underlying button.
    #[prop_or_default]
    pub r#type: ButtonType,
}

#[function_component(SimpleListItem)]
pub fn simple_list_item(props: &SimpleListItemProperties) -> Html {
    let class = classes!(props.class.clone(), "pf-v6-c-simple-list__item");
    let mut button_class = classes!(props.button_class.clone(), "pf-v6-c-simple-list__item-link");
    if props.active {
        button_class.push("pf-m-current");
    }
    html! {
        <li {class}>
            <button class={button_class} onclick={props.onclick.clone()} type="button">
                {props.children.clone()}
            </button>
        </li>
    }
}

/// A group organizing [`SimpleListItem`]s in a [`SimpleListGrouped`].
#[derive(Debug, Clone, PartialEq, Properties)]
pub struct SimpleListGroupProperties {
    /// The items that are part of this group.
    #[prop_or_default]
    pub children: ChildrenWithProps<SimpleListItem>,
    /// Additional classes to pass to the group.
    #[prop_or_default]
    pub class: Classes,
    /// Additional classes to pass to the title.
    #[prop_or_default]
    pub title_class: Classes,
    /// The title of the group.
    #[prop_or_default]
    pub title: Html,
}

#[function_component(SimpleListGroup)]
pub fn simple_list_group(props: &SimpleListGroupProperties) -> Html {
    let title_class = classes!(props.title_class.clone(), "pf-v6-c-simple-list__title");
    let class = classes!(props.class.clone(), "pf-v6-c-simple-list__list");
    html! {
        <section class="pf-v6-c-simple-list__section">
            <h2 class={title_class}>{props.title.clone()}</h2>
            <ul {class} role="list">
                {for props.children.iter()}
            </ul>
        </section>
    }
}
