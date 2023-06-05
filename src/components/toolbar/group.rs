use crate::{AsClasses, ToolbarElementModifier, WithBreakpoints};
use yew::prelude::*;

/// Properties for [`ToolbarGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarGroupProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,
}

/// A group item for a toolbar
#[function_component(ToolbarGroup)]
pub fn toolbar_group(props: &ToolbarGroupProperties) -> Html {
    let mut classes = Classes::from("pf-v5-c-toolbar__group");

    classes.extend(props.modifiers.as_classes());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
