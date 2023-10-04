use crate::prelude::{Cell, CellContext, MenuChildVariant, Span};

/// Render table entries
pub trait TableEntryRenderer<C>
where
    C: Clone + Eq + 'static,
{
    /// Render the cell for the requested column.
    fn render_cell(&self, context: CellContext<'_, C>) -> Cell;

    /// Control if the details section spans the full width.
    fn is_full_width_details(&self) -> Option<bool> {
        None
    }

    /// Render the details section.
    ///
    /// Used in combination with [`super::TableMode::Expandable`] or [`super::TableMode::CompactExpandable`].
    ///
    /// Defaults to not having details.
    fn render_details(&self) -> Vec<Span> {
        vec![]
    }

    /// Render the details section for a specific column.
    ///
    /// Used in combination with [`super::TableMode::Expandable`] or [`super::TableMode::CompactExpandable`] when one
    /// or more headers is marked `expandable=true`.
    ///
    /// Defaults to not having details.
    fn render_column_details(&self, #[allow(unused)] column: &C) -> Vec<Span> {
        vec![]
    }

    /// Render the row actions.
    ///
    /// Defaults to no actions.
    fn actions(&self) -> Vec<MenuChildVariant> {
        vec![]
    }
}
