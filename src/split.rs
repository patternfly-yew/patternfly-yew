use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SplitProperties {
    pub children: ChildrenWithProps<SplitItem>,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub wrap: bool,
}

/// The Split layout.
///
/// See: https://www.patternfly.org/v4/layouts/split
#[function_component(Split)]
pub fn split(props: &SplitProperties) -> Html {
    let mut classes = Classes::from("pf-l-split");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    if props.wrap {
        classes.push("pf-m-wrap");
    }

    html! (
        <div class={classes}>
        { for props.children.iter() }
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct SplitItemProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub fill: bool,
}

#[function_component(SplitItem)]
pub fn split_item(props: &SplitItemProperties) -> Html {
    let mut classes = Classes::from("pf-l-split__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    html!(
        <div class={classes}>
            { props.children.clone() }
        </div>
    )
}
