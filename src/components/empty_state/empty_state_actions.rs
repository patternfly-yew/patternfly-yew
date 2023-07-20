use crate::components::empty_state::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateActionsProperties {
    /** Content rendered inside the empty state actions */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the empty state actions */
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(EmptyStateActions)]
pub fn empty_state_actions(props: &EmptyStateActionsProperties) -> Html {
    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE_ACTIONS,
                props.class.clone(),
            )}
        >
            { for props.children.iter() }
        </div>
    )
}
