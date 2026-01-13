use std::ops::Deref;
use wasm_bindgen::JsCast;
use wasm_bindgen::prelude::*;
use web_sys::window;
use yew::prelude::*;

const CLICK_TYPE: &str = "mousedown";

/// Helper to trigger a close operation, when the user clicks on the global space.
///
/// This can be e.g. used for a drop down menu, where the component should be closed when the user
/// clicks outside of the dropped down content.
///
/// In order to use this, you need to define and assign the [`NodeRef`] to an element which is
/// considered the "inside". When the user clicks "outside" of the referenced element, it will
/// execute the callback.
///
/// When the instance is dropped, the callback will no longer be fired.
///
/// When creating the structure, you can pass in a new [`NodeRef`], and you can deref and clone
/// later using the function [`GlobalClose::node_ref`].
pub struct GlobalClose {
    node_ref: NodeRef,
    listener: Closure<dyn Fn(MouseEvent)>,
}

impl GlobalClose {
    pub fn new(node_ref: NodeRef, callback: Callback<()>) -> Self {
        let cloned_ref = node_ref.clone();
        let listener = Closure::wrap(Box::new(move |e: MouseEvent| {
            if let Some(control_ref) = cloned_ref.get() {
                if !control_ref.contains(e.target().as_ref().and_then(|t| t.dyn_ref())) {
                    callback.emit(());
                }
            }
        }) as Box<dyn Fn(MouseEvent)>);

        if let Some(cb) = listener.as_ref().dyn_ref() {
            window()
                .unwrap()
                .add_event_listener_with_callback(CLICK_TYPE, cb)
                .ok();
        }

        Self { node_ref, listener }
    }

    pub fn node_ref(&self) -> NodeRef {
        self.node_ref.clone()
    }
}

impl Drop for GlobalClose {
    fn drop(&mut self) {
        if let Some(cb) = self.listener.as_ref().dyn_ref() {
            window()
                .unwrap()
                .remove_event_listener_with_callback(CLICK_TYPE, cb)
                .ok();
        }
    }
}

impl Deref for GlobalClose {
    type Target = NodeRef;

    fn deref(&self) -> &Self::Target {
        &self.node_ref
    }
}
