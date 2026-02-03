//! Hooks for implementing pagination

use crate::prelude::Navigation;
use std::ops::{Deref, DerefMut, Range};
use std::rc::Rc;
use yew::prelude::*;

pub const DEFAULT_PER_PAGE: usize = 10;

/// The current control (input settings) of the pagination
#[derive(Copy, Clone, Debug, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PaginationControl {
    pub page: usize,
    pub per_page: usize,
}

/// The current state of the pagination
#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub struct PaginationState {
    pub control: PaginationControl,
    pub total: Option<usize>,
}

impl PaginationState {
    fn change_page(mut self, page: usize) -> Self {
        self.control.page = page;
        self
    }

    fn change_per_page(mut self, per_page: usize) -> Self {
        // remember the current offset
        let current_offset = self.control.page * self.control.per_page;

        self.control.per_page = per_page.max(1);

        // point to the page with the same offset as before
        self.control.page = current_offset / self.control.per_page;

        self
    }

    fn change_total(mut self, total: Option<usize>) -> Self {
        // set the new total
        self.total = total;

        // and check if we need to cap the current page
        if let Some(total_pages) = self.total_pages() {
            if total_pages > 0 {
                self.control.page = self.control.page.min(total_pages - 1);
            } else {
                self.control.page = 0;
            }
        }

        self
    }

    pub fn navigate(self, navigation: Navigation) -> Self {
        let mut newpage = self.control.page;
        match navigation {
            Navigation::First => newpage = 0,
            Navigation::Last => {
                if let Some(total_pages) = self.total_pages() {
                    newpage = total_pages.saturating_sub(1);
                }
            }
            Navigation::Next => {
                newpage = newpage.saturating_add(1);
                if let Some(total_pages) = self.total_pages() {
                    newpage = newpage.min(total_pages.max(1) - 1);
                }
            }
            Navigation::Previous => {
                newpage = newpage.saturating_sub(1);
            }
            Navigation::Page(page) => {
                if let Some(total_pages) = self.total_pages() {
                    if page < total_pages {
                        newpage = page;
                    }
                } else {
                    newpage = page;
                }
            }
        };

        self.change_page(newpage)
    }

    pub fn offset(&self) -> usize {
        self.control.per_page * self.control.page
    }

    pub fn range(&self) -> Range<usize> {
        let start = self.offset();
        let mut end = start + self.control.per_page;
        if let Some(total) = self.total {
            end = end.min(total);
        }

        Range { start, end }
    }

    pub fn total_pages(&self) -> Option<usize> {
        self.total
            .map(|total| total.div_ceil(self.control.per_page))
    }
}

impl Default for PaginationControl {
    fn default() -> Self {
        Self {
            page: 0,
            per_page: DEFAULT_PER_PAGE,
        }
    }
}

#[derive(Debug, PartialEq, Clone)]
pub struct UsePagination {
    pub state: UseStateHandle<PaginationState>,
    pub onnavigation: Callback<Navigation>,
    pub onperpagechange: Callback<usize>,
}

impl Deref for UsePagination {
    type Target = PaginationState;

    fn deref(&self) -> &Self::Target {
        &self.state
    }
}

impl Deref for PaginationState {
    type Target = PaginationControl;

    fn deref(&self) -> &Self::Target {
        &self.control
    }
}

impl DerefMut for PaginationState {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.control
    }
}

/// Create a hook for managing pagination state.
///
/// If known, the hook takes in a total number of items to be shown, otherwise it will be an
/// unbounded pagination control. The state will be initialized using the initializer function.
///
/// The hook returns a struct to manage and track pagination state. It is intended to be used
/// in combination with the [`crate::components::pagination::SimplePagination`] component.
///
/// ## Example
///
/// Also see the quickstart project for a full example.
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let total = use_state_eq(||Some(123));
///   let pagination = use_pagination(*total, Default::default);
///
///   html!(
///     <>
///       <SimplePagination
///         pagination={pagination.clone()}
///         total={*total}
///       />
///       // ... render content
///       { format!("Showing items: {:?}", pagination.state.range()) }
///       <SimplePagination
///         pagination={pagination.clone()}
///         total={*total}
///         position={PaginationPosition::Bottom}
///       />
///     </>
///   )
/// }
/// ```
#[hook]
pub fn use_pagination<T>(total: Option<usize>, init: T) -> UsePagination
where
    T: FnOnce() -> PaginationControl,
{
    let state = use_state_eq(|| PaginationState {
        control: init(),
        total,
    });

    use_effect_with((total, state.clone()), move |(total, state)| {
        state.set((**state).change_total(*total));
    });

    let onnavigation = use_callback(state.clone(), |nav: Navigation, state| {
        state.set((**state).navigate(nav))
    });

    let onperpagechange = use_callback(state.clone(), |per_page, state| {
        state.set((**state).change_per_page(per_page))
    });

    UsePagination {
        state,
        onnavigation,
        onperpagechange,
    }
}

/// Apply pagination state to a set of data.
///
/// Ideally, pagination is applied on the source of the data. Like a database query. However,
/// sometimes it can be convenient to even paginate a fully loaded dataset.
///
/// This hook takes a full dataset and returns the currently selected page. It will update
/// whenever the entries or pagination control state changes.
#[hook]
fn use_apply_pagination<T>(entries: Rc<Vec<T>>, control: PaginationControl) -> Rc<Vec<T>>
where
    T: Clone + PartialEq + 'static,
{
    use_memo((entries, control), |(entries, control)| {
        let offset = control.per_page * control.page;
        let limit = control.per_page;
        entries
            .iter()
            // apply pagination window
            .skip(offset)
            .take(limit)
            .cloned()
            .collect::<Vec<_>>()
    })
}

#[cfg(test)]
mod test {

    use super::*;

    fn state(page: usize, per_page: usize, total: Option<usize>) -> PaginationState {
        PaginationState {
            control: PaginationControl { per_page, page },
            total,
        }
    }

    #[test]
    fn test_navigate() {
        let state = state(0, 10, Some(23));
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);

        let state = state.navigate(Navigation::First);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);

        let state = state.navigate(Navigation::Last);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 2);
        assert_eq!(state.offset(), 20);
        assert_eq!(state.range(), 20..23);

        let state = state.navigate(Navigation::Previous);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 1);
        assert_eq!(state.offset(), 10);
        assert_eq!(state.range(), 10..20);

        let state = state.navigate(Navigation::Previous);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);
    }

    /// ensure that it's not possible to navigate before the first page
    #[test]
    fn test_underflow() {
        let state = state(0, 10, Some(23));

        let state = state.navigate(Navigation::Previous);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);
    }

    /// ensure start "next" stops with the last page
    #[test]
    fn test_overflow_1() {
        let state = state(0, 10, Some(23));

        let state = state.navigate(Navigation::Last);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 2);
        assert_eq!(state.offset(), 20);
        assert_eq!(state.range(), 20..23);

        let state = state.navigate(Navigation::Next);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 2);
        assert_eq!(state.offset(), 20);
        assert_eq!(state.range(), 20..23);
    }

    /// ensure that navigating beyond the last page doesn't work
    #[test]
    fn test_overflow_2() {
        let state = state(0, 10, Some(23));
        assert_eq!(state.total_pages(), Some(3));

        let state = state.navigate(Navigation::Page(5));
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);
    }

    #[test]
    fn test_change_page_size() {
        let state = state(0, 10, Some(23));
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);

        let state = state.navigate(Navigation::Next);
        assert_eq!(state.total_pages(), Some(3));
        assert_eq!(state.control.page, 1);
        assert_eq!(state.offset(), 10);
        assert_eq!(state.range(), 10..20);

        let state = state.change_per_page(5);
        assert_eq!(state.total_pages(), Some(5));
        assert_eq!(state.control.page, 2);
        assert_eq!(state.offset(), 10);
        assert_eq!(state.range(), 10..15);
    }

    #[test]
    fn test_change_none() {
        let state = state(0, 10, None);
        assert_eq!(state.total_pages(), None);
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..10);
    }

    #[test]
    fn test_change_empty() {
        let state = state(0, 10, Some(0));
        assert_eq!(state.total_pages(), Some(0));
        assert_eq!(state.control.page, 0);
        assert_eq!(state.offset(), 0);
        assert_eq!(state.range(), 0..0);
    }

    #[test]
    fn test_total_pages() {
        for i in 0..100 {
            let state = state(0, 10, Some(i));
            assert_eq!(
                state.total_pages(),
                Some((i as f64 / 10f64).ceil() as usize)
            );
        }
    }
}
