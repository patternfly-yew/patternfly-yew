use crate::prelude::*;
use gloo_utils::format::JsValueSerdeExt;
use serde_json::{json, Value};
use wasm_bindgen::prelude::*;

#[cfg_attr(debug_assertions, wasm_bindgen(module = "/js/debug/popperjs.js"))]
#[cfg_attr(
    not(debug_assertions),
    wasm_bindgen(module = "/js/release/popperjs.js")
)]
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

    styles.push_str(" z-index: 1000;");
    log::debug!(target: LOG_TARGET, "Computed Style: {styles}",);

    Ok(State {
        orientation,
        styles,
    })
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct Options {
    pub placement: Placement,
    pub strategy: Strategy,
    pub modifiers: Vec<Modifier>,
}

pub(crate) fn create_default_opts(apply: &Closure<dyn Fn(&Instance)>) -> Result<JsValue, JsValue> {
    create_opts(
        apply,
        Options {
            modifiers: vec![
                Modifier::Offset(Offset {
                    skidding: 0,
                    distance: 11,
                }),
                Modifier::PreventOverflow(PreventOverflow { padding: 0 }),
            ],
            placement: Placement::Auto,
            strategy: Strategy::Fixed,
        },
    )
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct Offset {
    pub skidding: i32,
    pub distance: i32,
}

impl Offset {
    pub fn to_json(&self) -> Value {
        json!({
            "offset": [self.skidding, self.distance],
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct PreventOverflow {
    pub padding: i32,
}

impl PreventOverflow {
    pub fn to_json(&self) -> Value {
        json!({
            "padding": self.padding,
        })
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum Modifier {
    Offset(Offset),
    PreventOverflow(PreventOverflow),
}

impl Modifier {
    pub fn to_json(&self) -> Value {
        match self {
            Self::Offset(options) => {
                json!({
                    "name": "offset",
                    "options": options.to_json(),
                })
            }
            Self::PreventOverflow(options) => {
                json!({
                    "name": "preventOverflow",
                    "options": options.to_json(),
                })
            }
        }
    }
}

pub(crate) fn create_opts(
    apply: &Closure<dyn Fn(&Instance)>,
    opts: Options,
) -> Result<JsValue, JsValue> {
    let mods = js_sys::Array::new();

    let m1 = js_sys::Object::new();
    js_sys::Reflect::set(&m1, &JsValue::from("name"), &JsValue::from("applyStyles"))?;
    js_sys::Reflect::set(&m1, &JsValue::from("phase"), &JsValue::from("write"))?;
    js_sys::Reflect::set(&m1, &JsValue::from("fn"), apply.as_ref())?;

    mods.push(&m1);

    for m in &opts.modifiers {
        let m =
            JsValue::from_serde(&m.to_json()).map_err(|err| JsValue::from_str(&err.to_string()))?;
        mods.push(&m);
    }

    let result = js_sys::Object::new();
    js_sys::Reflect::set(&result, &JsValue::from("modifiers"), &mods)?;
    js_sys::Reflect::set(
        &result,
        &JsValue::from("strategy"),
        &JsValue::from(opts.strategy.as_str()),
    )?;
    js_sys::Reflect::set(
        &result,
        &JsValue::from("placement"),
        &JsValue::from_str(opts.placement.as_str()),
    )?;

    web_sys::console::debug_2(&JsValue::from("options: "), &result);

    Ok(result.into())
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Placement {
    #[default]
    Auto,
    AutoStart,
    AutoEnd,

    Left,
    LeftStart,
    LeftEnd,

    Top,
    TopStart,
    TopEnd,

    Right,
    RightStart,
    RightEnd,

    Bottom,
    BottomStart,
    BottomEnd,
}

impl Placement {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Auto => "auto",
            Self::AutoStart => "auto-start",
            Self::AutoEnd => "auto-end",

            Self::Left => "left",
            Self::LeftStart => "left-start",
            Self::LeftEnd => "left-end",

            Self::Top => "top",
            Self::TopStart => "top-start",
            Self::TopEnd => "top-end",

            Self::Right => "right",
            Self::RightStart => "right-start",
            Self::RightEnd => "right-end",

            Self::Bottom => "bottom",
            Self::BottomStart => "bottom-start",
            Self::BottomEnd => "bottom-end",
        }
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum Strategy {
    #[default]
    Absolute,
    Fixed,
}

impl Strategy {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Absolute => "absolute",
            Self::Fixed => "fixed",
        }
    }
}
