use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
}

#[function_component(Stack)]
pub fn stack(props: &Props) -> Html {
    let mut classes = Classes::from("pf-l-stack");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct StackItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

#[function_component(StackItem)]
pub fn stack_item(props: &StackItemProps) -> Html {
    let mut classes = Classes::from("pf-l-stack__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    return html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    };
}
