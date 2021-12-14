use crate::{AsClasses, ToolbarElementModifier, WithBreakpoint};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarGroupProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: Vec<WithBreakpoint<ToolbarElementModifier>>,
}

#[function_component(ToolbarGroup)]
pub fn toolbar_group(props: &ToolbarGroupProps) -> Html {
    let mut classes = Classes::from("pf-c-toolbar__group");

    classes.extend(props.modifiers.as_classes());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
