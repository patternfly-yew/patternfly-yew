use crate::{AsClasses, ToolbarElementModifier, WithBreakpoints};
use yew::prelude::*;

/// Properties for [`ToolbarItem`]
#[derive(Clone, Eq, PartialEq, Debug)]
pub enum ToolbarItemType {
    None,
    BulkSelect,
    OverflowMenu,
    Pagination,
    SearchFilter,
}

impl Default for ToolbarItemType {
    fn default() -> Self {
        Self::None
    }
}

impl AsClasses for ToolbarItemType {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::None => {}
            Self::BulkSelect => classes.push("pf-m-bulk-select"),
            Self::OverflowMenu => classes.push("pf-m-overflow-menu"),
            Self::Pagination => classes.push("pf-m-pagination"),
            Self::SearchFilter => classes.push("pf-m-search-filter"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarItemProperties {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,
    #[prop_or_default]
    pub r#type: ToolbarItemType,
}

#[function_component(ToolbarItem)]
pub fn toolbar_item(props: &ToolbarItemProperties) -> Html {
    let mut classes = classes!("pf-c-toolbar__item");

    classes.extend(props.r#type.as_classes());
    classes.extend(props.modifiers.as_classes());

    html! (
        <div class={classes}>
            { for props.children.iter() }
        </div>
    )
}
