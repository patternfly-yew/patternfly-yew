use crate::prelude::{AsClasses, ToolbarElementModifier, WithBreakpoints};
use yew::prelude::*;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum GroupVariant {
    #[default]
    None,
    Button,
    Filter,
    IconButton,
}

impl AsClasses for GroupVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push(match self {
            Self::None => "",
            Self::Button => "pf-m-button-group",
            Self::Filter => "pf-m-filter-group",
            Self::IconButton => "pf-m-icon-button-group",
        });
    }
}

/// Properties for [`ToolbarGroup`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarGroupProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,
    #[prop_or_default]
    pub variant: GroupVariant,
}

/// A group item for a toolbar
#[function_component(ToolbarGroup)]
pub fn toolbar_group(props: &ToolbarGroupProperties) -> Html {
    let mut classes = Classes::from("pf-v5-c-toolbar__group");

    classes.extend(props.modifiers.as_classes());
    classes.extend(props.variant.as_classes());

    html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    }
}
