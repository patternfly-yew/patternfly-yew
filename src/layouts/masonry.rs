//! Masonry layout

use std::fmt::Write;

use crate::prelude::WithBreakpoints;
use yew::prelude::*;

/// Properties for [`Masonry`].
#[derive(Clone, PartialEq, Properties)]
pub struct MasonryProperties {
    pub children: Children,
    /// Adds gutter spacing between items.
    #[prop_or_default]
    pub gutter: bool,
    /// Number of columns, can be responsive via [`WithBreakpoints`].
    ///
    /// When set, this overrides `column_widths` and sets `column-count` inline.
    #[prop_or_default]
    pub columns: WithBreakpoints<AttrValue>,
    /// Sets the `column-width` CSS property, can be responsive via [`WithBreakpoints`].
    ///
    /// Used when `columns` is not set.
    #[prop_or_default]
    pub column_widths: WithBreakpoints<AttrValue>,
    /// Additional inline styles.
    #[prop_or_default]
    pub style: AttrValue,
    /// Additional classes.
    #[prop_or_default]
    pub class: Classes,
}

const COLUMNS: &str = "--pf-v6-l-masonry--Columns";
const COLUMN_WIDTH: &str = "--pf-v6-l-masonry--ColumnWidth";

/// Masonry layout
///
/// > Use a **Masonry** layout to arrange content in a Pinterest-style grid where
/// > items flow vertically into columns, filling available space efficiently.
///
/// The layout is implemented using CSS multi-column (`column-count` / `column-width`),
/// with children automatically wrapped in `pf-v6-l-masonry__item` containers.
///
/// See: <https://www.patternfly.org/layouts/masonry>
///
/// ## Properties
///
/// Defined by [`MasonryProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let columns = [
///     AttrValue::from("1").all(),
///     AttrValue::from("2").md(),
///     AttrValue::from("3").xl(),
///   ];
///
///   html!(
///     <Masonry gutter=true {columns}>
///       <div>{"Item 1"}</div>
///       <div>{"Item 2"}</div>
///       <div>{"Item 3"}</div>
///       <div>{"Item 4"}</div>
///     </Masonry>
///   )
/// }
/// ```
#[function_component(Masonry)]
pub fn masonry(props: &MasonryProperties) -> Html {
    let mut classes = classes!("pf-v6-l-masonry");

    if props.gutter {
        classes.push("pf-m-gutter");
    }
    classes.extend(props.class.clone());

    let mut style = String::new();
    for x in props.columns.iter() {
        let _ = write!(style, "{COLUMNS}{}: {};", x.on, x.modifier);
    }
    for x in props.column_widths.iter() {
        let _ = write!(style, "{COLUMN_WIDTH}{}: {};", x.on, x.modifier);
    }
    style.push_str(&props.style);

    html! {
        <div class={classes} style={style}>
            { for props.children.iter().map(|child| {
                html! {
                    <div class="pf-v6-l-masonry__item">
                        { child }
                    </div>
                }
            }) }
        </div>
    }
}
