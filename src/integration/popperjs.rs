use crate::Orientation;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::json;
use wasm_bindgen::prelude::*;

#[wasm_bindgen(module = "/src/js/popperjs.js")]
extern "C" {

    #[wasm_bindgen(js_name = "createPopper")]
    pub fn create_popper(
        reference: web_sys::Node,
        popper: web_sys::Node,
        opts: &JsValue,
    ) -> Instance;

    pub type Instance;

    #[wasm_bindgen(method)]
    pub fn destroy(this: &Instance);

    #[wasm_bindgen(method)]
    pub fn update(this: &Instance);

    #[wasm_bindgen(method, js_name = "forceUpdate")]
    pub fn force_update(this: &Instance);

}

const LOG_TARGET: &str = "popperjs";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct State {
    pub orientation: Orientation,
    pub styles: String,
}

pub(crate) fn from_popper(popper: &JsValue) -> Result<State, JsValue> {
    let state = js_sys::Reflect::get(popper, &JsValue::from("state"))?;
    let attributes = js_sys::Reflect::get(&state, &JsValue::from_str("attributes"))?;
    let popper = js_sys::Reflect::get(&attributes, &JsValue::from("popper"))?;
    let placement = js_sys::Reflect::get(&popper, &JsValue::from("data-popper-placement"))?;

    let orientation = match placement.as_string() {
        Some(p) if p == "bottom" => Orientation::Bottom,
        Some(p) if p == "top" => Orientation::Top,
        Some(p) if p == "left" => Orientation::Left,
        Some(p) if p == "right" => Orientation::Right,
        _ => Orientation::Bottom,
    };

    log::debug!(
        target: LOG_TARGET,
        "Orientation - original: {:?}, outcome: {:?}",
        placement.as_string(),
        orientation
    );

    let styles = js_sys::Reflect::get(&state, &JsValue::from_str("styles"))?;
    let popper = js_sys::Reflect::get(&styles, &JsValue::from("popper"))?;

    let popper = js_sys::Object::from(popper);

    let mut styles: String = js_sys::Object::entries(&popper)
        .to_vec()
        .iter()
        .map(js_sys::Array::from)
        .map(|field| {
            let key = js_sys::Array::get(&field, 0);
            let value = js_sys::Array::get(&field, 1);
            (
                key.as_string().unwrap_or_default(),
                value.as_string().unwrap_or_default(),
            )
        })
        .map(|(key, value)| format!("{}: {};", key, value))
        .collect::<Vec<String>>()
        .join(" ");

    styles.push_str("z-index: 1000;");

    Ok(State {
        orientation,
        styles,
    })
}

pub(crate) fn create_default_opts(apply: &Closure<dyn Fn(&Instance)>) -> Result<JsValue, JsValue> {
    let m1 = js_sys::Object::new();
    js_sys::Reflect::set(&m1, &JsValue::from("name"), &JsValue::from("applyStyles"))?;
    js_sys::Reflect::set(&m1, &JsValue::from("phase"), &JsValue::from("write"))?;
    js_sys::Reflect::set(&m1, &JsValue::from("fn"), apply.as_ref())?;

    let m2 = JsValue::from_serde(&json!({
        "name":"offset",
        "options": {
            "offset": [0,11],
        }
    }))
    .unwrap();

    let m3 = JsValue::from_serde(&json!({
        "name": "preventOverflow",
        "options": {
            "padding": 0,
        }
    }))
    .unwrap();

    let mods = js_sys::Array::of3(&m1, &m2, &m3);
    let opts = js_sys::Object::new();
    js_sys::Reflect::set(&opts, &JsValue::from("modifiers"), &mods)?;
    js_sys::Reflect::set(&opts, &JsValue::from("strategy"), &JsValue::from("fixed"))?;
    js_sys::Reflect::set(&opts, &JsValue::from("placement"), &JsValue::from("auto"))?;

    Ok(opts.into())
}
