use crate::components::empty_state::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateFooterProperties {
    /** Content rendered inside the empty state footer */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the empty state footer */
    #[prop_or_default]
    pub class: String,
}

#[function_component(EmptyStateFooter)]
pub fn empty_state_footer(props: &EmptyStateFooterProperties) -> Html {
    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE_FOOTER,
                &props.class,
            )}
        >
            { for props.children.iter() }
        </div>
    )
}
