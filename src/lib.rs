//! [PatternFly](https://patternfly.org) components implemented for [Yew](https://yew.rs).
//!
//! ## Goals
//!
//! This crate offers Yew components of PatternFly. The goal is to closely map functionality as
//! it exists in the ReactJS version, but take into account that Rust can sometimes do better. If
//! that is the case, and leads to a safer, more developer-friendly solution, it might be worth
//! doing things differently.
//!
//! The focus is currently on PatternFly v6.
//!
//! ## Help
//!
//! The `rustdoc` documentation should give you some detail information of the Rust code base. It
//! will not explain the components in detail, but offer links to the original PatternFly
//! documentation when possible. Those links are pointers to more information, for a better
//! understanding, but don't mean that they document PatternFly Yew in any way.
//!
//! Some components may offer an example section in their main component documentation. But for
//! more complex examples, see the
//! [PatternFly Yew Quickstart](https://github.com/ctron/patternfly-yew-quickstart) project.

#![recursion_limit = "1024"]
mod icon;

pub mod components;
pub mod core;
pub mod hooks;
pub mod layouts;
pub mod utils;
pub mod validation;

pub mod prelude;
