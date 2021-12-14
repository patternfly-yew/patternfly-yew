/*
 * Grow.on(Lg)
 */

use crate::AsClasses;
use std::fmt::Debug;
use std::ops::Deref;
use yew::html::IntoPropValue;
use yew::Classes;

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Breakpoint {
    None,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq,
{
    pub modifier: T,
    pub on: Breakpoint,
}

impl<T> WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn new(modifier: T) -> Self {
        Self {
            on: Breakpoint::None,
            modifier,
        }
    }

    pub fn map<R, F>(&self, f: F) -> WithBreakpoint<R>
    where
        R: Clone + Debug + PartialEq,
        F: Fn(&T) -> R,
    {
        WithBreakpoint {
            on: self.on,
            modifier: f(&self.modifier),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoints<T>(Vec<WithBreakpoint<T>>)
where
    T: Clone + Debug + PartialEq;

impl<T> std::default::Default for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    pub fn mapped<R, F>(&self, f: F) -> WithBreakpoints<R>
    where
        R: Clone + Debug + PartialEq,
        F: Fn(&T) -> R,
    {
        WithBreakpoints(self.0.iter().map(|i| i.map(|m| f(m))).collect::<Vec<_>>())
    }
}

impl<T> From<Vec<WithBreakpoint<T>>> for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    fn from(value: Vec<WithBreakpoint<T>>) -> Self {
        Self(value)
    }
}

impl<T> From<&[WithBreakpoint<T>]> for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    fn from(value: &[WithBreakpoint<T>]) -> Self {
        Self(Vec::from(value))
    }
}

impl<T, const N: usize> From<[WithBreakpoint<T>; N]> for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    fn from(value: [WithBreakpoint<T>; N]) -> Self {
        Self(Vec::from(value))
    }
}

impl<T> IntoIterator for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq,
{
    type Item = WithBreakpoint<T>;
    type IntoIter = std::vec::IntoIter<WithBreakpoint<T>>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl ToString for Breakpoint {
    fn to_string(&self) -> String {
        match self {
            Breakpoint::None => "",
            Breakpoint::Small => "-on-sm",
            Breakpoint::Medium => "-on-md",
            Breakpoint::Large => "-on-lg",
            Breakpoint::XLarge => "-on-xl",
            Breakpoint::XXLarge => "-on-2xl",
        }
        .to_string()
    }
}

pub trait WithBreakpointExt<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn on(&self, breakpoint: Breakpoint) -> WithBreakpoint<T>;

    fn all(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::None)
    }

    fn sm(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Small)
    }
    fn md(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Medium)
    }
    fn lg(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Large)
    }
    fn xl(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XLarge)
    }
    fn xxl(&self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XXLarge)
    }
}

impl<T> WithBreakpointExt<T> for T
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn on(&self, breakpoint: Breakpoint) -> WithBreakpoint<T> {
        WithBreakpoint {
            modifier: self.clone(),
            on: breakpoint,
        }
    }
}

impl<T> ToString for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn to_string(&self) -> String {
        format!("{}{}", self.modifier.to_string(), self.on.to_string())
    }
}

impl<T> Into<Classes> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into(self) -> Classes {
        Classes::from(self.to_string())
    }
}

impl<T> AsClasses for Vec<WithBreakpoint<T>>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn as_classes(&self) -> Classes {
        Classes::from(self.iter().map(|b| b.to_string()).collect::<Vec<_>>())
    }
}

impl<T> From<WithBreakpoints<T>> for Classes
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifiers: WithBreakpoints<T>) -> Self {
        let mods: Vec<_> = modifiers.0.into_iter().map(|b| b.to_string()).collect();
        Classes::from(mods)
    }
}

impl<T> IntoPropValue<Vec<WithBreakpoint<T>>> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into_prop_value(self) -> Vec<WithBreakpoint<T>> {
        vec![self]
    }
}

impl<T> Deref for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    type Target = Vec<WithBreakpoint<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifier: T) -> Self {
        Self::new(modifier)
    }
}

impl<T> From<WithBreakpoint<T>> for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifier: WithBreakpoint<T>) -> Self {
        WithBreakpoints(vec![modifier])
    }
}

impl<T> From<T> for WithBreakpoints<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifier: T) -> Self {
        WithBreakpoints(vec![modifier.into()])
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for Vec<WithBreakpoint<T>>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for &[WithBreakpoint<T>]
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T, const N: usize> IntoPropValue<WithBreakpoints<T>> for [WithBreakpoint<T>; N]
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T, const N: usize> IntoPropValue<WithBreakpoints<T>> for [T; N]
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into_iter()
            .map(|i| WithBreakpoint::new(i))
            .collect::<Vec<WithBreakpoint<T>>>()
            .into()
    }
}

/*
impl<T, const N: usize> Transformer<[T; N], WithBreakpoints<T>> for yew::virtual_dom::VComp
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn transform(from: [T; N]) -> WithBreakpoints<T> {
        WithBreakpoints(from.iter().map(|i| i.clone().into()).collect::<Vec<_>>())
    }
}

impl<T> Transformer<&[T], WithBreakpoints<T>> for yew::virtual_dom::VComp
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn transform(from: &[T]) -> WithBreakpoints<T> {
        WithBreakpoints(from.iter().map(|i| i.clone().into()).collect::<Vec<_>>())
    }
}
*/
