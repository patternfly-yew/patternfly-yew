//! Trait for rendering an item in a DualListSelector component.

use core::fmt::Debug;

use yew::prelude::*;

pub trait DualListSelectorItemRenderer: Debug + Clone + PartialEq + ToHtml + 'static {}

impl<T: ToHtml + Debug + Clone + PartialEq + 'static> DualListSelectorItemRenderer for T {}
