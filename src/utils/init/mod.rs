//! Initialization data

#[cfg(feature = "experimental")]
mod gen;

#[cfg(feature = "experimental")]
pub use gen::*;

/// A concept to provide an initial value.
///
/// In some cases it is necessary to provide an initial state. In some other cases it is necessary
/// to, later on, update that state. One way to achieve is this fully externalize the state,
/// and let the user deal with this. Another way is to let the component manage its own state and
/// allow the user to just provide an initial value. This latter approach has the downside that
/// it becomes hard overriding the state once it was initialized, as resetting the value to its
/// original state becomes a problem (since the initial value never changes).
///
/// A blanket implementation exists for every value implementing [`Clone`], [`Default`], and
/// [`PartialEq`] to also be an initial value of its own.
pub trait InitialValue<T>: Default + PartialEq {
    fn create(&self) -> T;
}

impl<T> InitialValue<T> for T
where
    T: Clone + Default + PartialEq,
{
    fn create(&self) -> T {
        self.clone()
    }
}
