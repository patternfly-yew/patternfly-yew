//! Masonry layout

use std::fmt::Write;

use crate::prelude::{Breakpoint, WithBreakpoints, use_id};
use yew::prelude::*;

/// Properties for [`Masonry`].
#[derive(Clone, PartialEq, Properties)]
pub struct MasonryProperties {
    pub children: Children,
    /// Adds gutter spacing between items (sets `column-gap`).
    #[prop_or_default]
    pub gutter: bool,
    /// Number of columns, can be responsive via [`WithBreakpoints`].
    ///
    /// Sets the CSS `column-count` property.
    #[prop_or_default]
    pub columns: WithBreakpoints<AttrValue>,
    /// Sets the CSS `column-width` property, can be responsive via [`WithBreakpoints`].
    ///
    /// When both are set, CSS limits the number of columns to `columns` while
    /// keeping each column at least this wide.
    #[prop_or_default]
    pub column_widths: WithBreakpoints<AttrValue>,
    /// Additional inline styles for the container.
    #[prop_or_default]
    pub style: AttrValue,
    /// Additional classes.
    #[prop_or_default]
    pub class: Classes,
}

fn breakpoint_min_width(bp: Breakpoint) -> Option<&'static str> {
    match bp {
        Breakpoint::Small => Some("36rem"),
        Breakpoint::Medium => Some("48rem"),
        Breakpoint::Large => Some("62rem"),
        Breakpoint::XLarge => Some("75rem"),
        Breakpoint::XXLarge => Some("90.625rem"),
        Breakpoint::None => None,
    }
}

/// Masonry layout
///
/// > Use a **Masonry** layout to arrange content in a Pinterest-style grid where
/// > items flow vertically into columns, filling available space efficiently.
///
/// Implemented using CSS multi-column (`column-count` / `column-width`).
/// Responsive breakpoint values are emitted as CSS `@media` rules in a `<style>` block.
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
    let id = use_id(None);
    let scope = format!("masonry-{id}");
    let mut classes = classes!("pf-v6-l-masonry");
    classes.push(scope.clone());

    if props.gutter {
        classes.push("pf-m-gutter");
    }
    classes.extend(props.class.clone());

    let mut inline_style = String::new();
    let media_css = column_css(&scope, &props.columns, &props.column_widths);
    let gap = if props.gutter {
        "var(--pf-t--global--spacer--md, 1rem)"
    } else {
        "0"
    };
    let _ = write!(inline_style, "column-gap: {gap};");
    let item_style = format!("break-inside: avoid; margin-bottom: {gap};");

    inline_style.push_str(&props.style);

    html! {
        <>
            if !media_css.is_empty() {
                <style>{ media_css }</style>
            }
            <div class={classes} style={inline_style}>
                { for props.children.iter().map(|child| {
                    html! {
                        <div class="pf-v6-l-masonry__item" style={item_style.clone()}>
                            { child }
                        </div>
                    }
                }) }
            </div>
        </>
    }
}

fn column_css(
    scope: &str,
    columns: &WithBreakpoints<AttrValue>,
    widths: &WithBreakpoints<AttrValue>,
) -> String {
    let mut css = String::new();
    // Ascending breakpoints make larger screens win regardless of prop order.
    for bp in [
        Breakpoint::None,
        Breakpoint::Small,
        Breakpoint::Medium,
        Breakpoint::Large,
        Breakpoint::XLarge,
        Breakpoint::XXLarge,
    ] {
        let mut declarations = String::new();
        for (property, values) in [("column-count", columns), ("column-width", widths)] {
            for value in values.iter().filter(|value| value.on == bp) {
                let _ = write!(declarations, "{property}: {};", value.modifier);
            }
        }
        if declarations.is_empty() {
            continue;
        }
        let rule = format!(".{scope} {{ {declarations} }}");
        if let Some(width) = breakpoint_min_width(bp) {
            let _ = write!(css, "@media (min-width: {width}) {{ {rule} }}");
        } else {
            css.push_str(&rule);
        }
    }
    css
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::prelude::WithBreakpointExt;

    #[test]
    fn responsive_columns_are_scoped_and_ordered() {
        let columns = [
            AttrValue::from("4").xl(),
            AttrValue::from("1").all(),
            AttrValue::from("2").md(),
        ]
        .into();
        let widths = AttrValue::from("200px").all().into();
        let css = column_css("masonry-first", &columns, &widths);
        assert_eq!(
            css,
            concat!(
                ".masonry-first { column-count: 1;column-width: 200px; }",
                "@media (min-width: 48rem) { .masonry-first { column-count: 2; } }",
                "@media (min-width: 75rem) { .masonry-first { column-count: 4; } }",
            )
        );
        assert!(!column_css("masonry-second", &columns, &widths).contains("masonry-first"));
    }
}
