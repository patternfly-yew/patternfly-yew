use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct LogoProperties {
    pub src: AttrValue,
    pub alt: AttrValue,
}

/// The Logo component using in the header navigation section.
#[function_component(Logo)]
pub fn logo(props: &LogoProperties) -> Html {
    html! (
        <img
            class="pf-c-brand"
            src={&props.src}
            alt={&props.alt}
        />
    )
}
