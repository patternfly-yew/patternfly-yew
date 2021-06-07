mod action;
mod breakpoint;
mod classes;
mod global_close;
mod orientation;
mod position;
mod size;
mod space;

pub use action::*;
pub use breakpoint::*;
pub use classes::*;
pub use global_close::*;
pub use orientation::*;
pub use position::*;
pub use size::*;
pub use space::*;

pub fn random_id() -> String {
    uuid::Uuid::new_v4().to_string()
}
