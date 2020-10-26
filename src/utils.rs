use std::fmt::{Debug, Formatter};

use wasm_bindgen::prelude::wasm_bindgen;
use wasm_bindgen::JsValue;

/// Something that can be selected.
pub struct Selected<T> {
    selected: bool,
    value: T,
}

/// Implement `Debug` if the value supports it.
impl<T> Debug for Selected<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Selected")
            .field("selected", &self.selected)
            .field("value", &self.value)
            .finish()
    }
}

impl<T> Clone for Selected<T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        Self {
            selected: self.selected,
            value: self.value.clone(),
        }
    }
}

#[wasm_bindgen(module = "/src/js/popperjs.js")]
extern "C" {
    pub fn createPopper(reference: web_sys::Node, popper: web_sys::Node, opts: &JsValue)
        -> JsValue;
}
