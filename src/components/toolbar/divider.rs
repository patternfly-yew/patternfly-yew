use yew::prelude::*;

/// A divider used in toolbars.
#[function_component(ToolbarDivider)]
pub fn divider() -> Html {
    html!(<hr class="pf-v5-c-divider pf-m-vertical" />)
}
