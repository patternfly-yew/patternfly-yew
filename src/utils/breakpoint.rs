/*
 * Grow.on(Lg)
 */

use crate::AsClasses;
use std::fmt::Debug;
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
    modifier: T,
    on: Breakpoint,
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
        self.iter()
            .map(|b| b.to_string())
            .collect::<String>()
            .into()
    }
}

impl<T> From<T> for WithBreakpoint<T>
where
    T: Clone + Debug + PartialEq + ToString,
{
    fn from(modifier: T) -> Self {
        Self {
            on: Breakpoint::None,
            modifier,
        }
    }
}
