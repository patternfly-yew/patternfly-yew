//! Skip to content

use yew::prelude::*;

/// Properties for [`SkipToContent`].
#[derive(Clone, PartialEq, Properties)]
pub struct SkipToContentProperties {
    /// The `href` of the skip link, typically a fragment identifier like `#main-content`.
    pub href: AttrValue,

    /// The link text, defaults to `"Skip to content"`.
    #[prop_or_default]
    pub children: Html,

    /// Additional classes for the wrapper.
    #[prop_or_default]
    pub class: Classes,

    /// Click handler, called before the default anchor navigation.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// Skip to content component
///
/// > A **skip to content** link allows keyboard users to bypass navigation and jump
/// > directly to the main content of the page. It is visually hidden until focused.
///
/// See: <https://www.patternfly.org/components/skip-to-content>
///
/// ## Properties
///
/// Defined by [`SkipToContentProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(
///         <SkipToContent href="#main-content">
///             { "Skip to main content" }
///         </SkipToContent>
///     )
/// }
/// ```
#[function_component(SkipToContent)]
pub fn skip_to_content(props: &SkipToContentProperties) -> Html {
    let (node_ref, href) = crate::utils::use_document_href(&props.href);
    let mut class = classes!("pf-v6-c-skip-to-content");
    class.extend(props.class.clone());

    let text = if props.children == Html::default() {
        html!({ "Skip to content" })
    } else {
        props.children.clone()
    };

    html! {
        <div {class}>
            <a
                ref={node_ref}
                class="pf-v6-c-button pf-m-primary"
                {href}
                onclick={props.onclick.clone()}
            >
                <span class="pf-v6-c-button__text">{ text }</span>
            </a>
        </div>
    }
}
