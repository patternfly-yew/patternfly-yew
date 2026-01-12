//! Trait for rendering an item in a DualListSelector component.

use core::fmt::Debug;

use yew::html::IntoPropValue;
use yew::prelude::*;

pub trait DualListSelectorItemRenderer:
    Debug + Clone + PartialEq + IntoPropValue<Html> + 'static
{
}

impl<T: IntoPropValue<Html> + Debug + Clone + PartialEq + 'static> DualListSelectorItemRenderer
    for T
{
}
