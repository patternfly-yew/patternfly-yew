use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct StackProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

/// The Stack layout.
///
/// See: https://www.patternfly.org/v4/layouts/stack
#[function_component(Stack)]
pub fn stack(props: &StackProperties) -> Html {
    let mut classes = Classes::from("pf-l-stack");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    html! (
        <div class={classes}>
            { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct StackItemProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

#[function_component(StackItem)]
pub fn stack_item(props: &StackItemProperties) -> Html {
    let mut classes = Classes::from("pf-l-stack__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    html! (
        <div class={classes}>
            { for props.children.iter() }
        </div>
    )
}
