//! Back to top

use crate::prelude::Icon;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;
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
    let visible = use_state(|| props.always_visible);
    let threshold = props.scroll_threshold;

    // Set up scroll listener
    {
        let visible = visible.clone();
        let always_visible = props.always_visible;

        use_effect_with(
            (always_visible, threshold),
            move |(always_visible, threshold)| {
                if *always_visible {
                    visible.set(true);
                    return Box::new(|| {}) as Box<dyn FnOnce()>;
                }

                let Some(win) = window() else {
                    return Box::new(|| {}) as Box<dyn FnOnce()>;
                };

                let toggle = {
                    let visible = visible.clone();
                    let threshold = *threshold;
                    let win = win.clone();
                    move || {
                        let scroll_y = win.scroll_y().unwrap_or(0.0);
                        visible.set(scroll_y > threshold);
                    }
                };

                // Initial check
                toggle();

                let cb = Closure::wrap(Box::new(toggle) as Box<dyn FnMut()>);
                if let Some(cb_ref) = cb.as_ref().dyn_ref() {
                    win.add_event_listener_with_callback("scroll", cb_ref).ok();
                }

                Box::new(move || {
                    if let Some(win) = window() {
                        if let Some(cb_ref) = cb.as_ref().dyn_ref() {
                            win.remove_event_listener_with_callback("scroll", cb_ref)
                                .ok();
                        }
                    }
                }) as Box<dyn FnOnce()>
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
        Callback::from(move |evt: MouseEvent| {
            // Scroll to top smoothly
            if let Some(win) = window() {
                let options = web_sys::ScrollToOptions::new();
                options.set_top(0.0);
                options.set_behavior(web_sys::ScrollBehavior::Smooth);
                win.scroll_to_with_scroll_to_options(&options);
            }
            user_onclick.emit(evt);
        })
    };

    html! {
        <div {class} {onclick}>
            <button class="pf-v6-c-button pf-m-primary" type="button">
                <span class="pf-v6-c-button__text">{ title }</span>
                <span class="pf-v6-c-button__icon pf-m-end">
                    { Icon::AngleUp }
                </span>
            </button>
        </div>
    }
}
