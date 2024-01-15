use std::sync::atomic::{AtomicUsize, Ordering};

pub struct Ouia(&'static str);

impl Ouia {
    pub const fn new(component_name: &'static str) -> Self {
        Self(component_name)
    }

    pub fn generated_id(&self) -> String {
        let count = counter();
        format!("OUIA-Generated-{}-{count}", self.0)
    }

    pub fn component_type(&self) -> String {
        format!("PF5/{}", self.0)
    }
}

fn counter() -> usize {
    static COUNT: AtomicUsize = AtomicUsize::new(0);
    COUNT.fetch_add(1, Ordering::Relaxed)
}
