//! Level

use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LayoutLevelProperties {
    #[prop_or_default]
    pub children: Html,
    /// Adds space between children.
    #[prop_or_default]
    pub gutter: bool,
    /// Additional classes added to the level.
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub id: Option<AttrValue>,
}

/// Level layout
///
/// Named `LayoutLevel` to avoid clashing with the title [`crate::prelude::Level`].
///
/// > Use a **Level** layout to distribute items horizontally across a container, with equal space between items.
/// > On small viewports, items are stacked vertically.
///
/// See: <https://www.patternfly.org/layouts/level>
///
/// ## Properties
///
/// Defined by [`LayoutLevelProperties`].
///
/// ## Children
///
/// The level layout is supposed to contain [`crate::prelude::LevelItem`] children.
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
///     <LayoutLevel gutter=true>
///       <LevelItem>{"Left"}</LevelItem>
///       <LevelItem>{"Center"}</LevelItem>
///       <LevelItem>{"Right"}</LevelItem>
///     </LayoutLevel>
///   )
/// }
/// ```
#[function_component(LayoutLevel)]
pub fn layout_level(props: &LayoutLevelProperties) -> Html {
    let mut classes = classes!("pf-v6-l-level", props.class.clone());

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    html!(
        <div id={props.id.clone()} class={classes}>
            { props.children.clone() }
        </div>
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct LevelItemProperties {
    #[prop_or_default]
    pub children: Html,
    /// Additional classes added to the level item.
    #[prop_or_default]
    pub class: Classes,
}

/// An item in the [`LayoutLevel`] layout.
///
/// ## Properties
///
/// Defined by [`LevelItemProperties`].
#[function_component(LevelItem)]
pub fn level_item(props: &LevelItemProperties) -> Html {
    html!(
        <div class={classes!("pf-v6-l-level__item", props.class.clone())}>
            { props.children.clone() }
        </div>
    )
}
