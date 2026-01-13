//! Hooks for handling IDs

use crate::prelude::{Id, random_id};
use yew::prelude::*;

/// Use a random ID
#[hook]
pub fn use_random_id() -> UseStateHandle<Id> {
    use_state_eq(Id::new)
}

/// Use an ID from properties, or random if none was provided
///
/// This value will not change when re-rendering.
#[hook]
pub fn use_prop_id<I>(id: I) -> std::rc::Rc<String>
where
    I: Into<Option<String>>,
{
    use_memo(id.into(), |id| id.clone().unwrap_or_else(random_id))
}

/// Use an ID from properties, or random if none was provided
///
/// This value will not change when re-rendering.
#[hook]
pub fn use_id<I>(id: I) -> AttrValue
where
    I: Into<Option<AttrValue>>,
{
    let s = use_memo(id.into(), |id| {
        id.clone().unwrap_or_else(|| AttrValue::from(random_id()))
    });
    (*s).clone()
}

/// Use an ID, derived from another one.
///
/// It can be used in combination with [`use_id`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[derive(PartialEq, Properties)]
/// struct Properties {
///     id: Option<AttrValue>,
/// }
///
/// #[function_component(Example)]
/// fn component(props: &Properties) -> Html {
///     let id = use_id(props.id.clone());
///     let child_id = use_derived_id(&id, |id| format!("{id}-child"));
///
///     html!({"..."})
/// }
/// ```
///
/// This value will not change when re-rendering.
#[hook]
pub fn use_derived_id<F>(id: &AttrValue, f: F) -> AttrValue
where
    F: FnOnce(&str) -> String,
{
    let s = use_memo(id.clone(), |id| AttrValue::from(f(id)));
    (*s).clone()
}

/// Use an ID, derived from another one with a suffix.
///
/// It can be used in combination with [`use_id`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[derive(PartialEq, Properties)]
/// struct Properties {
///     id: Option<AttrValue>,
/// }
///
/// #[function_component(Example)]
/// fn component(props: &Properties) -> Html {
///     let id = use_id(props.id.clone());
///     let child_id = use_suffixed_id(&id, "-child");
///
///     html!({"..."})
/// }
/// ```
///
/// This value will not change when re-rendering.
#[hook]
pub fn use_suffixed_id(id: &AttrValue, suffix: &str) -> AttrValue {
    let s = use_memo(id.clone(), |id| AttrValue::from(id.to_string() + suffix));
    (*s).clone()
}
