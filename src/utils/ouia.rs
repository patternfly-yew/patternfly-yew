use std::sync::atomic::{AtomicUsize, Ordering};
use yew::AttrValue;
use yew::html::IntoPropValue;

#[macro_export]
macro_rules! ouia {
    ($framework:literal, $component:literal) => {
        Ouia::with_full(concat!($framework, "/", $component))
    };
    ($component:literal) => {
        ouia!("PF5", $component)
    };
}

pub struct Ouia(OuiaComponentType);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OuiaComponentType(&'static str);

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct OuiaSafe(bool);

impl OuiaSafe {
    pub const TRUE: OuiaSafe = OuiaSafe(true);
    pub const FALSE: OuiaSafe = OuiaSafe(false);
}

impl From<bool> for OuiaSafe {
    fn from(value: bool) -> Self {
        Self(value)
    }
}

impl IntoPropValue<OuiaSafe> for bool {
    fn into_prop_value(self) -> OuiaSafe {
        OuiaSafe(self)
    }
}

impl IntoPropValue<AttrValue> for OuiaSafe {
    fn into_prop_value(self) -> AttrValue {
        match self.0 {
            true => AttrValue::Static("true"),
            false => AttrValue::Static("false"),
        }
    }
}

impl IntoPropValue<Option<AttrValue>> for OuiaSafe {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.into_prop_value())
    }
}

impl IntoPropValue<AttrValue> for OuiaComponentType {
    fn into_prop_value(self) -> AttrValue {
        AttrValue::Static(self.0)
    }
}

impl IntoPropValue<Option<AttrValue>> for OuiaComponentType {
    fn into_prop_value(self) -> Option<AttrValue> {
        Some(self.into_prop_value())
    }
}

impl Ouia {
    pub const fn with_full(full_component_name: &'static str) -> Self {
        Self(OuiaComponentType(full_component_name))
    }

    pub fn generated_id(&self) -> String {
        let count = counter();
        format!("OUIA-Generated-{count}")
    }

    pub const fn component_type(&self) -> OuiaComponentType {
        self.0
    }
}

fn counter() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    COUNT.fetch_add(1, Ordering::Relaxed)
}
