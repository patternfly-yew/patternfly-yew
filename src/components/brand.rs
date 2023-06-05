//! Brand visual
use yew::prelude::*;

#[deprecated(since = "0.4.0", note = "Was renamed to `Brand` component")]
pub type Logo = Brand;

/// Properties for [`Brand`]
#[derive(Clone, PartialEq, Properties)]
pub struct BrandProperties {
    pub src: AttrValue,
    pub alt: AttrValue,
}

/// Brand component
///
/// > A **brand** is used to place a product logotype on a screen.
///
/// See: <https://www.patternfly.org/v4/components/brand>
///
/// ## Properties
///
/// Defined by [`BrandProperties`].
#[function_component(Brand)]
pub fn brand(props: &BrandProperties) -> Html {
    html! (
        <img
            class="pf-v5-c-brand"
            src={&props.src}
            alt={&props.alt}
        />
    )
}
