use crate::prelude::*;
use yew::prelude::*;

/// Properties for [`SimplePagination`].
#[derive(Debug, PartialEq, Properties)]
pub struct SimplePaginationProperties {
    /// The pagination state, created by [`crate::hooks::pagination::use_pagination`].
    pub pagination: UsePagination,

    /// Choices for number of items per page.
    #[prop_or(vec![10,25,50])]
    pub entries_per_page_choices: Vec<usize>,

    /// Total number of items, if known.
    #[prop_or_default]
    pub total: Option<usize>,

    /// Position of the pagination control
    #[prop_or(PaginationPosition::Top)]
    pub position: PaginationPosition,

    /// Element ID.
    #[prop_or_default]
    pub id: Option<AttrValue>,

    /// Additional styles
    #[prop_or_default]
    pub style: AttrValue,

    /// Disable the control
    #[prop_or_default]
    pub disabled: bool,
}

/// An opinionated pagination component.
///
/// This component provides an opinionated approach to pagination, using the state from
/// [`crate::hooks::pagination::use_pagination`] and the more basic
/// [`crate::components::pagination::Pagination`] component.
///
/// For an example see [`crate::hooks::pagination::use_pagination`] or the quickstart project.
#[function_component(SimplePagination)]
pub fn simple_pagination(props: &SimplePaginationProperties) -> Html {
    let total = &props.total;
    let UsePagination {
        state,
        onnavigation,
        onperpagechange,
    } = props.pagination.clone();

    html!(
        <Pagination
            total_entries={*total}
            offset={(*state).offset()}
            entries_per_page_choices={props.entries_per_page_choices.clone()}
            selected_choice={(*state).control.per_page}
            onlimit={&onperpagechange}
            onnavigation={&onnavigation}
            style={&props.style}
            id={&props.id}
            disabled={props.disabled}
        />
    )
}
