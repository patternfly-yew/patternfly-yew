//! Stack

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StackProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub gutter: bool,
}

/// Stack layout
///
/// > Use a Stack layout to position items vertically in a container, with one item filling the available vertical space.
///
/// See: <https://www.patternfly.org/layouts/stack>
///
/// ## Properties
///
/// Defined by [`StackProperties`].
///
/// ## Children
///
/// The stack layout is supposed to contain [`StackItem`] children. However, there is no restriction
/// through component types on that.
#[function_component(Stack)]
pub fn stack(props: &StackProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-stack");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    html! (
        <div class={classes}>
            { props.children.clone() }
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct StackItemProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub fill: bool,
}

/// An item in the [`Stack`] layout.
///
/// ## Properties
///
/// Defined by [`StackItemProperties`].
#[function_component(StackItem)]
pub fn stack_item(props: &StackItemProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-stack__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    html! (
        <div class={classes}>
            { props.children.clone() }
        </div>
    )
}
