//! Gallery

use crate::prelude::wrap::wrapper_div_with_attributes;
use yew::prelude::*;
use yew::virtual_dom::AttributeOrProperty;

#[derive(Clone, PartialEq, Properties)]
pub struct GalleryProperties {
    pub children: Children,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub style: AttrValue,
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
#[function_component(Gallery)]
pub fn gallery(props: &GalleryProperties) -> Html {
    let mut classes = classes!("pf-v5-l-gallery");

    if props.gutter {
        classes.push(classes!("pf-m-gutter"));
    }

    html! (
        <div
            class={classes}
            style={&props.style}
        >
        { for props.children.iter().map(|child|{
            wrapper_div_with_attributes(child, &[("class", AttributeOrProperty::Static("pf-v5-l-gallery__item"))])
        }) }
        </div>
    )
}
