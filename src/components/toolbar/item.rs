use crate::prelude::{AsClasses, ExtendClasses, ToolbarElementModifier, WithBreakpoints};
use yew::prelude::*;

/// Properties for [`ToolbarItem`]
#[derive(Clone, Default, Eq, PartialEq, Debug)]
pub enum ToolbarItemType {
    #[default]
    None,
    BulkSelect,
    ChipGroup,
    OverflowMenu,
    Pagination,
    SearchFilter,
}

impl AsClasses for ToolbarItemType {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::None => {}
            Self::BulkSelect => classes.push("pf-m-bulk-select"),
            Self::ChipGroup => classes.push("pf-m-chip-group"),
            Self::OverflowMenu => classes.push("pf-m-overflow-menu"),
            Self::Pagination => classes.push("pf-m-pagination"),
            Self::SearchFilter => classes.push("pf-m-search-filter"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarItemProperties {
    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub modifiers: WithBreakpoints<ToolbarElementModifier>,

    #[prop_or_default]
    pub r#type: ToolbarItemType,

    /// Control the width of the item
    #[prop_or_default]
    pub width: WithBreakpoints<String>,

    /// Control the minimal width of the item
    #[prop_or_default]
    pub min_width: WithBreakpoints<String>,

    /// Additional classes
    #[prop_or_default]
    pub class: Classes,
}

#[function_component(ToolbarItem)]
pub fn toolbar_item(props: &ToolbarItemProperties) -> Html {
    let mut class = classes!("pf-v6-c-toolbar__item");

    class.extend_from(&props.r#type);
    class.extend_from(&props.modifiers);
    class.extend(props.class.clone());

    let style = props
        .width
        .iter()
        .map(|w| format!("--pf-v6-c-toolbar__item--Width{}: {};", w.on, w.modifier))
        .chain(
            props
                .min_width
                .iter()
                .map(|w| format!("--pf-v6-c-toolbar__item--MinWidth{}: {};", w.on, w.modifier)),
        )
        .collect::<String>();

    html! (
        <div
            id={&props.id}
            {class}
            {style}
        >
            { props.children.clone() }
        </div>
    )
}
