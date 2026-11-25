//! Back to top

use crate::prelude::{Align, Button, ButtonVariant, Icon};
use gloo_events::{EventListener, EventListenerOptions};
use yew::prelude::*;

/// Properties for [`BackToTop`].
#[derive(Clone, PartialEq, Properties)]
pub struct BackToTopProperties {
    /// The button label, defaults to `"Back to top"`.
    #[prop_or_default]
    pub title: Option<AttrValue>,

    /// Whether the button is always visible, regardless of scroll position.
    ///
    /// When `false` (the default), the button only appears after the user has
    /// scrolled down past the threshold.
    #[prop_or_default]
    pub always_visible: bool,

    /// Scroll threshold in pixels before the button becomes visible.
    #[prop_or(400.0)]
    pub scroll_threshold: f64,

    /// Additional classes for the wrapper.
    #[prop_or_default]
    pub class: Classes,

    /// Click handler, called in addition to the default scroll-to-top behavior.
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
}

/// Back to top component
///
/// > A **back to top** button appears as a sticky button on the lower right of the
/// > page after the user scrolls down. Clicking it scrolls back to the top.
///
/// Inside a [`crate::prelude::Page`], this scrolls the page main element.
/// Otherwise, it scrolls the document.
///
/// See: <https://www.patternfly.org/components/back-to-top>
///
/// ## Properties
///
/// Defined by [`BackToTopProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///     html!(<BackToTop />)
/// }
/// ```
#[function_component(BackToTop)]
pub fn back_to_top(props: &BackToTopProperties) -> Html {
    let node_ref = use_node_ref();
    let visible = use_state_eq(|| props.always_visible);

    {
        let visible = visible.clone();
        let node_ref = node_ref.clone();
        use_effect_with(
            (props.always_visible, props.scroll_threshold),
            move |(always_visible, threshold)| {
                let listener = if *always_visible {
                    visible.set(true);
                    None
                } else {
                    scroll_container(&node_ref).and_then(|target| {
                        let win = target.owner_document()?.default_view()?;
                        let threshold = *threshold;
                        visible.set(f64::from(target.scroll_top()) > threshold);
                        // Element scroll events do not bubble; capture also observes Page scrolling.
                        Some(EventListener::new_with_options(
                            &win,
                            "scroll",
                            EventListenerOptions::run_in_capture_phase(),
                            move |_| visible.set(f64::from(target.scroll_top()) > threshold),
                        ))
                    })
                };
                move || drop(listener)
            },
        );
    }

    let mut class = classes!("pf-v6-c-back-to-top");
    if !*visible {
        class.push("pf-m-hidden");
    }
    class.extend(props.class.clone());

    let title = props
        .title
        .clone()
        .unwrap_or_else(|| AttrValue::from("Back to top"));

    let onclick = {
        let user_onclick = props.onclick.clone();
        let node_ref = node_ref.clone();
        Callback::from(move |evt: MouseEvent| {
            // Scroll to top smoothly
            if let Some(target) = scroll_container(&node_ref) {
                let options = web_sys::ScrollToOptions::new();
                options.set_top(0.0);
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                target.scroll_to_with_scroll_to_options(&options);
            }
            user_onclick.emit(evt);
        })
    };

    html! {
        <div ref={node_ref} {class} style="position: fixed;">
            <Button
                variant={ButtonVariant::Primary}
                {onclick}
                icon={Icon::AngleUp.as_html()}
                align={Align::End}
            >
                { title }
            </Button>
        </div>
    }
}

fn scroll_container(node_ref: &NodeRef) -> Option<web_sys::Element> {
    let node = node_ref.cast::<web_sys::Element>()?;
    node.closest(".pf-v6-c-page__main")
        .ok()
        .flatten()
        .or_else(|| node.owner_document()?.scrolling_element())
}
