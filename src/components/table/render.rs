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
    /// Defaults to not having details.
    fn render_details(&self) -> Vec<Span> {
        vec![]
    }

    /// Render the row actions.
    ///
    /// Defaults to no actions.
    fn actions(&self) -> Vec<MenuChildVariant> {
        vec![]
    }
}
