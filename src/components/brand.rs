//! Brand visual
use yew::prelude::*;

/// Properties for [`Brand`]
#[derive(PartialEq, Properties)]
pub struct BrandProperties {
    pub src: AttrValue,
    pub alt: AttrValue,

    /// Additional style
    #[prop_or_default]
    pub style: AttrValue,

    #[prop_or_default]
    pub children: ChildrenWithProps<BrandSource>,
}

#[derive(Clone, PartialEq, Properties)]
pub struct BrandSourceProperties {
    #[prop_or_default]
    /// media selector
    pub media: Option<String>,
    /// source
    pub srcset: AttrValue,
}

#[function_component(BrandSource)]
pub fn brand_source(props: &BrandSourceProperties) -> Html {
    html!(
        <source
            media={props.media.clone()}
            srcset={&props.srcset}
        />
    )
}

/// Brand component
///
/// > A **brand** is used to place a product logotype on a screen.
///
/// See: <https://www.patternfly.org/components/brand>
///
/// Both basic and responsive modes are supported. If the list of sources is empty, then the basic
/// mode is used. Otherwise it will use the response mode, with the default image as fallback.
///
/// The additional style will either be applied to the plain `img` element, or to the `picture`
/// element in case of the responsive mode.
///
/// ## Properties
///
/// Defined by [`BrandProperties`].
#[function_component(Brand)]
pub fn brand(props: &BrandProperties) -> Html {
    if props.children.is_empty() {
        html! (
            <img
                class="pf-v6-c-brand"
                style={&props.style}
                src={&props.src}
                alt={&props.alt}
            />
        )
    } else {
        html! (
            <picture
                class="pf-v6-c-brand pf-m-picture"
                style={&props.style}
            >
                { for props.children.iter() }
                <img
                    class="pf-v6-c-brand"
                    src={&props.src}
                    alt={&props.alt}
                />
            </picture>
        )
    }
}
