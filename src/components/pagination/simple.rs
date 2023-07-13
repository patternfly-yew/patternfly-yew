use crate::prelude::*;
use yew::prelude::*;

#[derive(Debug, PartialEq, Properties)]
pub struct SimplePaginationProperties {
    /// The pagination state, created by [`crate::hooks::pagination::use_pagination`].
    pub pagination: UsePagination,

    #[prop_or(vec![10,25,50])]
    pub entries_per_page_choices: Vec<usize>,

    /// Total number of items, if known.
    #[prop_or_default]
    pub total: Option<usize>,

    #[prop_or(PaginationPosition::Top)]
    pub position: PaginationPosition,
}

#[function_component(SimplePagination)]
pub fn simple_pagination(props: &SimplePaginationProperties) -> Html {
    let total = &props.total;
    let UsePagination {
        state,
        onpagechange,
        onperpagechange,
    } = props.pagination.clone();

    html!(
        <Pagination
            total_entries={*total}
            offset={(*state).offset()}
            entries_per_page_choices={props.entries_per_page_choices.clone()}
            selected_choice={(*state).control.per_page}
            onlimit={&onperpagechange}
            onnavigation={&onpagechange}
        />
    )
}
