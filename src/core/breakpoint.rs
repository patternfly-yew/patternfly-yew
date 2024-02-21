/*
 * Grow.on(Lg)
 */

use crate::prelude::AsClasses;
use std::fmt::{Debug, Display, Formatter};
use std::ops::Deref;
use yew::html::IntoPropValue;
use yew::prelude::*;
use yew_more_hooks::prelude::Breakpoint as BreakpointTrait;

/// Breakpoint definitions
#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum Breakpoint {
    None,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

impl BreakpointTrait for Breakpoint {
    fn as_pixels(&self) -> usize {
        match self {
            Breakpoint::None => 0,
            Breakpoint::Small => 576,
            Breakpoint::Medium => 768,
            Breakpoint::Large => 992,
            Breakpoint::XLarge => 1200,
            Breakpoint::XXLarge => 1450,
        }
    }

    fn from_screen_width(pixels: usize) -> Self {
        match pixels {
            w if w < Breakpoint::Small.as_pixels() => Breakpoint::None,
            w if w < Breakpoint::Medium.as_pixels() => Breakpoint::Small,
            w if w < Breakpoint::Large.as_pixels() => Breakpoint::Medium,
            w if w < Breakpoint::XLarge.as_pixels() => Breakpoint::Large,
            w if w < Breakpoint::XXLarge.as_pixels() => Breakpoint::XLarge,
            _ => Breakpoint::XXLarge,
        }
    }
}

#[hook]
pub fn use_breakpoint() -> UseStateHandle<Breakpoint> {
    yew_more_hooks::prelude::use_breakpoint()
}

/// A combination of a style/variant for a specific [`Breakpoint`].
#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoint<T>
where
    T: PartialEq,
{
    pub modifier: T,
    pub on: Breakpoint,
}

impl<T: Eq> Eq for WithBreakpoint<T> {}

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

/// A set of variants for breakpoints.
///
/// This is typically used by components which support different variants for different breakpoints,
/// including a default one.
#[derive(Clone, Debug, PartialEq)]
pub struct WithBreakpoints<T>(Vec<WithBreakpoint<T>>)
where
    T: PartialEq;

impl<T: Eq> Eq for WithBreakpoints<T> {}

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
    fn extend_classes(&self, classes: &mut Classes) {
        AsClasses::extend_classes(&self.0, classes)
    }
}

impl<T> AsClasses for WithBreakpoint<T>
where
    T: PartialEq + AsClasses,
{
    fn extend_classes(&self, classes: &mut Classes) {
        // get as classes, but then extend but the breakpoint rules
        classes.extend(
            self.modifier
                .as_classes()
                .into_iter()
                .map(|c| format!("{}{}", c, self.on)),
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
                .map(|i| i.map(&f))
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

impl Display for Breakpoint {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        match self {
            Breakpoint::None => f.write_str(""),
            Breakpoint::Small => f.write_str("-on-sm"),
            Breakpoint::Medium => f.write_str("-on-md"),
            Breakpoint::Large => f.write_str("-on-lg"),
            Breakpoint::XLarge => f.write_str("-on-xl"),
            Breakpoint::XXLarge => f.write_str("-on-2xl"),
        }
    }
}

/// Helps creating [`WithBreakpoint`] instances.
///
/// ## Example
///
/// The following example populates the `cols` attribute of a `GridItem`. I requires a number of
/// columns to span, which can be specified for the different breakpoints.
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   html!(
///     <Grid>
///         <GridItem cols={[4.all(), 8.lg()]}>{"cell"}</GridItem>
///     </Grid>
///   )
/// }
/// ```
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
            .map(WithBreakpoint::new)
            .collect::<Vec<WithBreakpoint<T>>>()
            .into()
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for &[T]
where
    T: PartialEq + Clone,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.iter()
            .map(|i| WithBreakpoint::new(i.clone()))
            .collect::<Vec<WithBreakpoint<T>>>()
            .into()
    }
}

impl<T> IntoPropValue<WithBreakpoints<T>> for Vec<T>
where
    T: PartialEq,
{
    fn into_prop_value(self) -> WithBreakpoints<T> {
        self.into_iter()
            .map(WithBreakpoint::new)
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
    use crate::prelude::{AsClasses, WithBreakpoints};
    use yew::prelude::*;

    use super::*;

    #[derive(Copy, Clone, Debug, PartialEq, Eq)]
    pub enum MockVariant {
        Foo,
        Bar,
        Baz,
    }

    #[derive(PartialEq, Properties)]
    struct MockComponentProperties {
        pub variant: WithBreakpoints<MockVariant>,
    }

    #[function_component(MockComponent)]
    fn component(_props: &MockComponentProperties) -> Html {
        html!()
    }

    impl AsClasses for MockVariant {
        fn extend_classes(&self, classes: &mut Classes) {
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

    #[test]
    fn test_map() {
        let prop: WithBreakpoints<bool> = [true.all(), true.lg()].into();
        assert_eq!(
            prop.mapped(|f| f.then(|| "static".to_string()))
                .as_classes(),
            Classes::from("static static-on-lg")
        );
    }

    #[test]
    fn compiles_assign_array() {
        let _ = html!(<MockComponent variant={[MockVariant::Foo]} />);
        let _ = html!(<MockComponent variant={[MockVariant::Foo, MockVariant::Bar]} />);
    }
}
