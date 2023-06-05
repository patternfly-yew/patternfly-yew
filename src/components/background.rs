//! Background image
use yew::prelude::*;

/// Properties for [`Background`]
#[derive(Clone, PartialEq, Properties)]
pub struct BackgroundProperties {
    #[prop_or_default]
    pub filter: Option<String>,
}

/// Background image component
///
/// > A **background image** allows you to place an image in the background of your page or area of a page.
///
/// See: <https://www.patternfly.org/v4/components/background-image>
///
/// ## Properties
///
/// Defined by [`BackgroundProperties`].
#[function_component(Background)]
pub fn view(props: &BackgroundProperties) -> Html {
    if let Some(filter) = &props.filter {
        let styles = format!("--pf-v5-c-background-image--Filter: {};", filter);
        html! (
            <div class="pf-v5-c-background-image" style={styles}></div>
        )
    } else {
        // FIXME: something is still wrong here, the filter gets applied, but seems to have no effect
        html! (
            <div class="pf-v5-c-background-image">
                <svg xmlns="http://www.w3.org/2000/svg" class="pf-v5-c-background-image__filter" width="0" height="0">
                    <filter id="image_overlay">
                        <feColorMatrix type="matrix" values="1 0 0 0 0 1 0 0 0 0 1 0 0 0 0 0 0 0 1 0"></feColorMatrix>
                        <feComponentTransfer color-interpolation-filters="sRGB" result="duotone">
                            <feFuncR type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncR>
                            <feFuncG type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncG>
                            <feFuncB type="table" tableValues="0.086274509803922 0.43921568627451"></feFuncB>
                            <feFuncA type="table" tableValues="0 1"></feFuncA>
                        </feComponentTransfer>
                    </filter>
                </svg>
            </div>
        )
    }
}
