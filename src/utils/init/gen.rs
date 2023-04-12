use super::InitialValue;
use std::{fmt::Formatter, ops::Deref};
use uuid::Uuid;

/// An initial value with a generation.
///
/// This value is equal to its value and generation. Getting a reference or mapping the value
/// doesn't create a new generation. However, creating a new value does.
///
/// It implements [`InitialValue`], so that setting a new generational value will trigger a
/// re-initialization of the state.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Generational<T> {
    value: T,
    generation: Uuid,
}

impl<T> Generational<T> {
    pub fn new(value: T) -> Self {
        Self {
            value,
            generation: Uuid::new_v4(),
        }
    }

    pub fn as_ref(&self) -> Generational<&T> {
        Generational {
            value: &self.value,
            generation: self.generation,
        }
    }

    pub fn map<F, U>(self, f: F) -> Generational<U>
    where
        F: FnOnce(T) -> U,
    {
        Generational {
            value: f(self.value),
            generation: self.generation,
        }
    }
}

impl<T> InitialValue<T> for Generational<T>
where
    T: Clone + Default + PartialEq,
{
    fn create(&self) -> T {
        self.value.clone()
    }
}

impl<T> Deref for Generational<T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.value
    }
}

impl<T> From<T> for Generational<T> {
    fn from(value: T) -> Self {
        Self::new(value)
    }
}

impl<T> std::fmt::Display for Generational<T>
where
    T: std::fmt::Display,
{
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        self.value.fmt(f)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_eq() {
        let value = Generational::new(0);

        // same same
        assert_eq!(value, value);
        // Clone must yields an equal instance
        assert_eq!(value, value.clone());
    }

    #[test]
    fn test_ne() {
        // value differs
        assert_ne!(Generational::new(0), Generational::new(1));
        // generation differs
        assert_ne!(Generational::new(0), Generational::new(0));
    }

    #[test]
    fn test_map() {
        let value = Generational::new(0);

        // must be the same, as map keeps the generation
        assert_eq!(value, value.as_ref().map(|r| *r));

        // must be different, as the generation is the same, but the value is not
        assert_ne!(value, value.as_ref().map(|r| *r + 1));
    }
}
