use crate::{AsClasses, ToolbarElementModifier, WithBreakpoints};
use yew::prelude::*;

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
    fn as_classes(&self) -> Classes {
        match self {
            Self::None => Classes::new(),
            Self::BulkSelect => "pf-m-bulk-select".into(),
            Self::OverflowMenu => "pf-m-overflow-menu".into(),
            Self::Pagination => "pf-m-pagination".into(),
            Self::SearchFilter => "pf-m-search-filter".into(),
        }
    }

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
pub struct ToolbarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,
    #[prop_or_default]
    pub r#type: ToolbarItemType,
}

#[function_component(ToolbarItem)]
pub fn toolbar_item(props: &ToolbarItemProps) -> Html {
    let mut classes = Classes::from("pf-c-toolbar__item");

    classes.extend(props.r#type.as_classes());
    classes.extend(props.modifiers.as_classes());

    return html! {
        <div class={classes}>
            { for props.children.iter() }
        </div>
    };
}
