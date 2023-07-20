use crate::components::empty_state::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateBodyProperties {
    /** Content rendered inside the empty state body */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the empty state body */
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(EmptyStateBody)]
pub fn empty_state_body(props: &EmptyStateBodyProperties) -> Html {
    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE_BODY,
                props.class.clone(),
            )}
        >
            { for props.children.iter() }
        </div>
    )
}
