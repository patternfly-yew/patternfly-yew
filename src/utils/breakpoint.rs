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
    T: PartialEq,
{
    pub modifier: T,
    pub on: Breakpoint,
}

impl<T> WithBreakpoint<T>
where
    T: PartialEq,
{
    pub fn new(modifier: T) -> Self {
        Self {
            on: Breakpoint::None,
            modifier,
        }
    }

    pub fn map<R, F>(self, f: F) -> WithBreakpoint<R>
    where
        R: PartialEq,
        F: Fn(T) -> R,
    {
        WithBreakpoint {
            on: self.on,
            modifier: f(self.modifier),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoints<T>(Vec<WithBreakpoint<T>>)
where
    T: PartialEq;

impl<T> Default for WithBreakpoints<T>
where
    T: PartialEq,
{
    fn default() -> Self {
        Self(vec![])
    }
}

impl<T> AsClasses for WithBreakpoints<T>
where
    T: PartialEq + AsClasses,
{
    fn extend(&self, classes: &mut Classes) {
        AsClasses::extend(&self.0, classes)
    }
}

impl<T> AsClasses for WithBreakpoint<T>
where
    T: PartialEq + AsClasses,
{
    fn extend(&self, classes: &mut Classes) {
        // get as classes, but then extend but the breakpoint rules
        classes.extend(
            self.modifier
                .as_classes()
                .into_iter()
                .map(|c| format!("{}{}", c, self.on.to_string())),
        )
    }
}

impl<T> WithBreakpoints<T>
where
    T: Clone + PartialEq,
{
    pub fn mapped<R, F>(&self, f: F) -> WithBreakpoints<R>
    where
        R: PartialEq,
        F: Fn(T) -> R,
    {
        WithBreakpoints(
            self.0
                .clone()
                .into_iter()
                .map(|i| i.map(|m| f(m)))
                .collect::<Vec<_>>(),
        )
    }
}

impl<T> From<Vec<WithBreakpoint<T>>> for WithBreakpoints<T>
where
    T: PartialEq,
{
    fn from(value: Vec<WithBreakpoint<T>>) -> Self {
        Self(value)
    }
}

impl<T> From<&[WithBreakpoint<T>]> for WithBreakpoints<T>
where
    T: Clone + PartialEq,
{
    fn from(value: &[WithBreakpoint<T>]) -> Self {
        Self(Vec::from(value))
    }
}

impl<T, const N: usize> From<[WithBreakpoint<T>; N]> for WithBreakpoints<T>
where
    T: PartialEq,
{
    fn from(value: [WithBreakpoint<T>; N]) -> Self {
        Self(Vec::from(value))
    }
}

impl<T> IntoIterator for WithBreakpoints<T>
where
    T: PartialEq,
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

pub trait WithBreakpointExt<T>: Sized
where
    T: PartialEq,
{
    fn on(self, breakpoint: Breakpoint) -> WithBreakpoint<T>;

    fn all(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::None)
    }
    fn sm(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Small)
    }
    fn md(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Medium)
    }
    fn lg(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::Large)
    }
    fn xl(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XLarge)
    }
    fn xxl(self) -> WithBreakpoint<T> {
        self.on(Breakpoint::XXLarge)
    }
}

impl<T> WithBreakpointExt<T> for T
where
    T: PartialEq,
{
    fn on(self, breakpoint: Breakpoint) -> WithBreakpoint<T> {
        WithBreakpoint {
            modifier: self,
            on: breakpoint,
        }
    }
}

impl<T> IntoPropValue<Vec<WithBreakpoint<T>>> for WithBreakpoint<T>
where
    T: PartialEq,
{
    fn into_prop_value(self) -> Vec<WithBreakpoint<T>> {
        vec![self]
    }
}

impl<T> Deref for WithBreakpoints<T>
where
    T: PartialEq,
{
    type Target = Vec<WithBreakpoint<T>>;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<T> From<T> for WithBreakpoint<T>
where
    T: PartialEq,
{
    fn from(modifier: T) -> Self {
        Self::new(modifier)
    }
}

impl<T> From<WithBreakpoint<T>> for WithBreakpoints<T>
where
    T: PartialEq,
{
    fn from(modifier: WithBreakpoint<T>) -> Self {
        WithBreakpoints(vec![modifier])
    }
}

impl<T> From<T> for WithBreakpoints<T>
where
    T: PartialEq,
{
    fn from(modifier: T) -> Self {
        WithBreakpoints(vec![modifier.into()])
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for WithBreakpoint<T>
where
    T: PartialEq,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        vec![self].into()
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for Vec<WithBreakpoint<T>>
where
    T: PartialEq,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for &[WithBreakpoint<T>]
where
    T: Clone + PartialEq,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T, const N: usize> IntoPropValue<WithBreakpoints<T>> for [WithBreakpoint<T>; N]
where
    T: PartialEq,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into()
    }
}

impl<T, const N: usize> IntoPropValue<WithBreakpoints<T>> for [T; N]
where
    T: PartialEq,
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

#[cfg(test)]
mod test {
    use crate::{AsClasses, WithBreakpoints};
    use yew::Classes;

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum MockVariant {
        Foo,
        Bar,
        Baz,
    }

    impl AsClasses for MockVariant {
        fn extend(&self, classes: &mut Classes) {
            match self {
                Self::Foo => {}
                Self::Bar => classes.push("bar"),
                Self::Baz => classes.push("foo bar"),
            }
        }
    }

    #[test]
    fn test_empty_string() {
        let prop: WithBreakpoints<String> = [].into();
        assert_eq!(prop.as_classes(), Classes::from(""));
    }

    #[test]
    fn test_single_string() {
        let prop: WithBreakpoints<String> = "foo".to_string().into();
        assert_eq!(prop.as_classes(), Classes::from("foo"));
    }

    #[test]
    fn test_single_with_string() {
        let prop: WithBreakpoints<String> = "foo".to_string().xxl().into();
        assert_eq!(prop.as_classes(), Classes::from("foo-on-2xl"));
    }

    #[test]
    fn test_some_string() {
        let prop: WithBreakpoints<String> =
            ["one".to_string().all(), "two".to_string().xxl()].into();
        assert_eq!(prop.as_classes(), Classes::from("one two-on-2xl"));
    }

    #[test]
    fn test_some_variant() {
        let prop: WithBreakpoints<MockVariant> = [MockVariant::Foo.all()].into();
        assert_eq!(prop.as_classes(), Classes::from(""));

        let prop: WithBreakpoints<MockVariant> = [MockVariant::Bar.all()].into();
        assert_eq!(prop.as_classes(), Classes::from("bar"));

        let prop: WithBreakpoints<MockVariant> = [MockVariant::Baz.all()].into();
        assert_eq!(prop.as_classes(), Classes::from("foo bar"));
    }
}
