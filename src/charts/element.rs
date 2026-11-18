//! Building React elements from Rust.

use super::sys;
use js_sys::{Array, Object, Reflect};
use serde::Serialize;
use std::any::Any;
use wasm_bindgen::{JsValue, closure::Closure};
use yew::AttrValue;

/// Keeps the Rust closures, handed over to JavaScript, alive while they are referenced by the
/// rendered chart.
#[derive(Default)]
pub struct BuildContext {
    closures: Vec<Box<dyn Any>>,
}

impl BuildContext {
    /// Create a JavaScript function taking one argument.
    pub fn function1(&mut self, f: impl Fn(JsValue) -> JsValue + 'static) -> JsValue {
        let closure = Closure::<dyn Fn(JsValue) -> JsValue>::new(f);
        let value = closure.as_ref().clone();
        self.closures.push(Box::new(closure));
        value
    }

    /// Create a JavaScript function taking two arguments.
    pub fn function2(&mut self, f: impl Fn(JsValue, JsValue) -> JsValue + 'static) -> JsValue {
        let closure = Closure::<dyn Fn(JsValue, JsValue) -> JsValue>::new(f);
        let value = closure.as_ref().clone();
        self.closures.push(Box::new(closure));
        value
    }
}

/// A value which can be converted into a JavaScript value.
pub trait IntoJs {
    fn into_js(self) -> JsValue;
}

macro_rules! into_js {
    ($($t:ty),*) => {
        $(impl IntoJs for $t {
            fn into_js(self) -> JsValue {
                JsValue::from(self)
            }
        })*
    };
}

into_js!(JsValue, f64, u32, bool, &str, String);

impl IntoJs for AttrValue {
    fn into_js(self) -> JsValue {
        JsValue::from_str(self.as_str())
    }
}

/// Something which can be turned into a React element.
pub trait ToElement {
    fn to_element(&self, ctx: &mut BuildContext) -> JsValue;
}

/// Convert a serializable value into a JavaScript value.
pub fn to_js<T: Serialize + ?Sized>(value: &T) -> JsValue {
    serde_json::to_string(value)
        .ok()
        .and_then(|json| js_sys::JSON::parse(&json).ok())
        .unwrap_or(JsValue::UNDEFINED)
}

/// Read a property of a JavaScript object.
pub fn get(target: &JsValue, key: &str) -> JsValue {
    if target.is_object() {
        Reflect::get(target, &JsValue::from_str(key)).unwrap_or(JsValue::UNDEFINED)
    } else {
        JsValue::UNDEFINED
    }
}

/// Builder for a React element.
pub struct Element {
    r#type: JsValue,
    props: Object,
    children: Array,
}

impl Element {
    /// A PatternFly chart component, by its exported name (e.g. `ChartBar`).
    pub fn new(name: &str) -> Self {
        Self::of_type(JsValue::from_str(name))
    }

    /// A component by its type.
    pub fn of_type(r#type: JsValue) -> Self {
        Self {
            r#type,
            props: Object::new(),
            children: Array::new(),
        }
    }

    pub fn set(&mut self, key: &str, value: impl IntoJs) -> &mut Self {
        let _ = Reflect::set(&self.props, &JsValue::from_str(key), &value.into_js());
        self
    }

    pub fn opt(&mut self, key: &str, value: Option<impl IntoJs>) -> &mut Self {
        if let Some(value) = value {
            self.set(key, value);
        }
        self
    }

    pub fn str(&mut self, key: &str, value: &Option<AttrValue>) -> &mut Self {
        if let Some(value) = value {
            self.set(key, value.as_str());
        }
        self
    }

    pub fn json<T: Serialize + ?Sized>(&mut self, key: &str, value: &T) -> &mut Self {
        self.set(key, to_js(value))
    }

    pub fn opt_json<T: Serialize>(&mut self, key: &str, value: &Option<T>) -> &mut Self {
        if let Some(value) = value {
            self.json(key, value);
        }
        self
    }

    /// Set a list, unless it is empty.
    pub fn list<T: Serialize>(&mut self, key: &str, value: &[T]) -> &mut Self {
        if !value.is_empty() {
            self.json(key, value);
        }
        self
    }

    /// Set a nested element.
    pub fn element(
        &mut self,
        key: &str,
        value: &Option<impl ToElement>,
        ctx: &mut BuildContext,
    ) -> &mut Self {
        if let Some(value) = value {
            let value = value.to_element(ctx);
            self.set(key, value);
        }
        self
    }

    /// Merge additional, raw, properties. Those take precedence.
    pub fn extra(&mut self, extra: &Option<serde_json::Value>) -> &mut Self {
        if let Some(serde_json::Value::Object(map)) = extra {
            for (key, value) in map {
                self.json(key, value);
            }
        }
        self
    }

    pub fn child(&mut self, child: JsValue) -> &mut Self {
        if !child.is_null() && !child.is_undefined() {
            self.children.push(&child);
        }
        self
    }

    pub fn build(&self) -> JsValue {
        match sys::create_element(&self.r#type, &self.props, &self.children) {
            Ok(element) => element,
            Err(err) => {
                log::error!("Failed to create chart element: {err:?}");
                JsValue::NULL
            }
        }
    }
}
