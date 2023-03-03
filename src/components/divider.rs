use yew::prelude::*;

/// Divider component
///
/// > A **divider** is a horizontal or vertical line that is placed between screen elements to create visual divisions and content groupings.
///
/// See: <https://www.patternfly.org/v4/components/divider>
///
/// This component is normally used as part of a list of items, like as part of the
/// [`AppLauncher`](crate::prelude::AppLauncher).
///
/// ## Properties
///
/// This component does not have properties.
#[function_component(Divider)]
pub fn divider() -> Html {
    html! {<li class="pf-c-divider" role="separator"></li>}
}
