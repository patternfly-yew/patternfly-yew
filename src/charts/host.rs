//! Hosting a React tree inside a Yew component.

use super::element::{BuildContext, ToElement};
use super::sys;
use wasm_bindgen::JsValue;
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub(crate) struct ChartHostProperties<P: ToElement + Clone + PartialEq + 'static> {
    pub element: P,
}

struct HostState {
    root: JsValue,
    // Closures referenced by the currently rendered tree.
    _current: BuildContext,
}

/// Renders a chart element into a React root, attached to a `div` managed by Yew.
#[function_component(ChartHost)]
pub(crate) fn chart_host<P>(props: &ChartHostProperties<P>) -> Html
where
    P: ToElement + Clone + PartialEq + 'static,
{
    let node = use_node_ref();
    let state = use_mut_ref(|| None::<HostState>);

    {
        let node = node.clone();
        let state = state.clone();
        use_effect_with(props.element.clone(), move |element| {
            let mut ctx = BuildContext::default();
            let element = element.to_element(&mut ctx);

            let mut state = state.borrow_mut();
            if state.is_none()
                && let Some(container) = node.cast::<web_sys::Element>()
            {
                match sys::create_root(&container) {
                    Ok(root) => {
                        *state = Some(HostState {
                            root,
                            _current: BuildContext::default(),
                        })
                    }
                    Err(err) => log::error!("Failed to create chart root: {err:?}"),
                }
            }

            if let Some(state) = state.as_mut() {
                // rendering is synchronous, so the previous closures can be released afterward
                if let Err(err) = sys::render_root(&state.root, &element) {
                    log::error!("Failed to render chart: {err:?}");
                }
                state._current = ctx;
            }
        });
    }

    {
        let state = state.clone();
        use_effect_with((), move |()| {
            move || {
                if let Some(state) = state.borrow_mut().take()
                    && let Err(err) = sys::unmount_root(&state.root)
                {
                    log::error!("Failed to unmount chart: {err:?}");
                }
            }
        });
    }

    html!(<div ref={node} />)
}
