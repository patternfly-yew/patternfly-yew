#[cfg(feature = "yew-nested-router")]
mod router;
mod simple;

#[cfg(feature = "yew-nested-router")]
pub use router::*;
pub use simple::*;
