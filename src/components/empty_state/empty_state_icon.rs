use crate::components::empty_state::styles::*;
use std::fmt::Debug;

use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateIconProperties {
    /** Additional classes added to the empty state icon */
    #[prop_or_default]
    pub class: AttrValue,

    /** Adds an accessible name for the Table */
    #[prop_or_default]
    pub icon: Children,

    /** Changes the color of the icon.  */
    #[prop_or_default]
    pub color: String,
}

#[function_component(EmptyStateIcon)]
pub fn empty_state_icon(props: &EmptyStateIconProperties) -> Html {
    let icon = props.icon.iter().map(|mut vnode| {
        if let yew::virtual_dom::VNode::VTag(tag) = &mut vnode {
            tag.add_attribute("class", props.class.clone());
            tag.add_attribute("aria-hidden", "true");
        }
        vnode
    });

    let style = classes!(conditional!(
        !props.color.is_empty(),
        format!(
            "{}: {};",
            EmptyStateStyles::VARIABLE_C_EMPTY_STATE_ICON_COLOR,
            props.color
        )
    ));

    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE_ICON,
            )}
            {style}
        >
            { for icon }
        </div>
    )
}
