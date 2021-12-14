use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct Props {
    pub children: ChildrenWithProps<SplitItem>,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub wrap: bool,
}

#[function_component(Split)]
pub fn split(props: &Props) -> Html {
    let mut classes = Classes::from("pf-l-split");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    if props.wrap {
        classes.push("pf-m-wrap");
    }

    html! {
        <div class={classes}>
        { for props.children.iter().map(|child|{
            html!{ {child} }
        }) }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct SplitItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

#[function_component(SplitItem)]
pub fn split_item(props: &SplitItemProps) -> Html {
    let mut classes = Classes::from("pf-l-split__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    return html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    };
}
