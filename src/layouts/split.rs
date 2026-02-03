//! Split

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct SplitProperties {
    pub children: ChildrenWithProps<SplitItem>,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub wrap: bool,
}

/// Split layout
///
/// > Use a **Split** layout to position items horizontally in a container, with one item filling the remaining horizontal space as the viewport is resized.
///
/// See: <https://www.patternfly.org/layouts/split>
///
/// ## Properties
///
/// Defined by [`SplitProperties`].
///
/// ## Children
///
/// The grid layout is supposed to contain [`crate::prelude::SplitItem`] children.
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   html!(
///     <Split gutter=true>
///       <SplitItem>{"Foo"}</SplitItem>
///       <SplitItem fill=true>{"Full Width"}</SplitItem>
///     </Split>
///   )
/// }
/// ```
#[function_component(Split)]
pub fn split(props: &SplitProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-split");

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
    pub children: Html,
    #[prop_or_default]
    pub fill: bool,
}

/// An item in the [`Split`] layout.
///
/// ## Properties
///
/// Defined by [`SplitItemProperties`].
#[function_component(SplitItem)]
pub fn split_item(props: &SplitItemProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-split__item");

    if props.fill {
        classes.push("pf-m-fill");
    }

    html!(
        <div class={classes}>
            { props.children.clone() }
        </div>
    )
}
