//! Bindings to the bundled JavaScript module (React + PatternFly Victory charts).

use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/charts/js/patternfly-charts.js")]
extern "C" {
    #[wasm_bindgen(catch)]
    pub fn create_root(container: &web_sys::Element) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub fn render_root(root: &JsValue, element: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn unmount_root(root: &JsValue) -> Result<(), JsValue>;

    #[wasm_bindgen(catch)]
    pub fn container_type(first: &str, second: &str) -> Result<JsValue, JsValue>;

    #[wasm_bindgen(catch)]
    pub fn create_element(
        r#type: &JsValue,
        props: &JsValue,
        children: &js_sys::Array,
    ) -> Result<JsValue, JsValue>;
}
