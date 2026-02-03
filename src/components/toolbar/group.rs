use crate::prelude::{AsClasses, ExtendClasses, ToolbarElementModifier, WithBreakpoints};
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
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,
    #[prop_or_default]
    pub variant: GroupVariant,

    /// Additional classes
    #[prop_or_default]
    pub class: Classes,
}

/// A group item for a toolbar
#[function_component(ToolbarGroup)]
pub fn toolbar_group(props: &ToolbarGroupProperties) -> Html {
    let mut class = Classes::from("pf-v6-c-toolbar__group");

    class.extend_from(&props.modifiers);
    class.extend_from(&props.variant);
    class.extend(props.class.clone());

    html! {
        <div
            id={&props.id}
            {class}
        >
            { props.children.clone() }
        </div>
    }
}
