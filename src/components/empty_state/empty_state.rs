use crate::components::empty_state::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum EmptyStateVariant {
    XS,
    SM,
    LG,
    XL,
    FULL,
}

impl EmptyStateVariant {
    pub fn classes(&self, full_height: bool) -> &str {
        return match self {
            Self::XS => EmptyStateStyles::MODIFIERS_XS,
            Self::SM => EmptyStateStyles::MODIFIERS_SM,
            Self::LG => EmptyStateStyles::MODIFIERS_LG,
            Self::XL => EmptyStateStyles::MODIFIERS_XL,
            Self::FULL => {
                if full_height {
                    EmptyStateStyles::MODIFIERS_FULL_HEIGHT
                } else {
                    ""
                }
            }
        };
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateProperties {
    /** Content rendered inside the empty state */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the empty state */
    #[prop_or_default]
    pub class: String,

    /** Modifies empty state max-width and sizes of icon, title and body */
    #[prop_or(EmptyStateVariant::FULL)]
    pub variant: EmptyStateVariant,

    /** Cause component to consume the available height of its container */
    #[prop_or_default]
    pub full_height: bool,
}

#[function_component(EmptyState)]
pub fn empty_state(props: &EmptyStateProperties) -> Html {
    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE,
                props.variant.classes(props.full_height).to_string(),
                &props.class,
            )}
        >
            <div
                class={classes!(
                    EmptyStateStyles::EMPTY_STATE_CONTENT,
                )}
            >
                { for props.children.iter() }
            </div>
        </div>
    )
}
