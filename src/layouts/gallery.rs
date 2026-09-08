//! Gallery

use std::fmt::Write;

use crate::prelude::{Breakpoint, WithBreakpoints, wrap::wrapper_div_with_attributes};
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;

#[derive(Clone, PartialEq, Properties)]
pub struct GalleryProperties {
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub style: AttrValue,
    #[prop_or_default]
    pub min_widths: WithBreakpoints<AttrValue>,
    #[prop_or_default]
    pub max_widths: WithBreakpoints<AttrValue>,
}

const MIN: &str = "--pf-v6-l-gallery--GridTemplateColumns--min";
const MAX: &str = "--pf-v6-l-gallery--GridTemplateColumns--max";

impl Breakpoint {
    fn suffix(&self) -> &'static str {
        match self {
            Breakpoint::None => "",
            Breakpoint::Small => "-on-sm",
            Breakpoint::Medium => "-on-md",
            Breakpoint::Large => "-on-lg",
            Breakpoint::XLarge => "-on-xl",
            Breakpoint::XXLarge => "-on-2xl",
        }
    }
}

/// Gallery layout
///
/// > Use a **Gallery** layout to arrange content in a responsive grid. Content will wrap responsively to create uniform rows and columns.
///
/// See: <https://www.patternfly.org/layouts/gallery>
///
/// ## Properties
///
/// Defined by [`GalleryProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let min_widths = [
///     AttrValue::from("100%").all(),
///     AttrValue::from("100px").md(),
///     AttrValue::from("300px").xl()
///   ];
///   let max_widths = [
///     AttrValue::from("200px").md(),
///     AttrValue::from("1fr").xl()
///   ];
///
///   html!(
///     <Gallery gutter=true {min_widths} {max_widths} >
///       <div>{"Gallery Item"}</div>
///       <div>{"Gallery Item"}</div>
///       <div>{"Gallery Item"}</div>
///       <div>{"Gallery Item"}</div>
///     </Gallery>
///   )
/// }
/// ```
#[function_component(Gallery)]
pub fn gallery(props: &GalleryProperties) -> Html {
    let mut classes = classes!("pf-v6-l-gallery");

    if props.gutter {
        classes.push(classes!("pf-m-gutter"));
    }

    let mut style = String::new();
    for (property, widths) in [(MIN, &props.min_widths), (MAX, &props.max_widths)] {
        for x in widths.iter() {
            let _ = write!(style, "{property}{}: {};", x.on.suffix(), x.modifier);
        }
    }
    style.push_str(&props.style);

    html! (
        <div class={classes} style={style}>
            { for props.children.iter().map(|child|{
            wrapper_div_with_attributes(child, &[("class", AttributeOrProperty::Static("pf-v6-l-gallery__item"))])
        }) }
        </div>
    )
}
