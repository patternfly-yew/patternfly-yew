use crate::{AsClasses, ToolbarElementModifier, WithBreakpoint};
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
}

#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarItemProps {
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub modifiers: Vec<WithBreakpoint<ToolbarElementModifier>>,
    #[prop_or_default]
    pub r#type: ToolbarItemType,
}

pub struct ToolbarItem {
    props: ToolbarItemProps,
}

impl Component for ToolbarItem {
    type Message = ();
    type Properties = ToolbarItemProps;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if self.props != props {
            self.props = props;
            true
        } else {
            false
        }
    }

    fn view(&self) -> Html {
        let mut classes = Classes::from("pf-c-toolbar__item");

        classes = classes.extend(self.props.r#type.as_classes());
        classes = classes.extend(self.props.modifiers.as_classes());

        return html! {
            <div class=classes>
                { for self.props.children.iter() }
            </div>
        };
    }
}
