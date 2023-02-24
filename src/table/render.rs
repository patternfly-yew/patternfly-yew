use super::{Cell, Span};
use crate::{CellContext, DropdownChildVariant};
use yew::prelude::*;

#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct ColumnIndex {
    pub index: usize,
}

/// Render table entries
#[deprecated(
    since = "0.4.0",
    note = "Please implement `TableEntryRenderer` instead."
)]
pub trait TableRenderer {
    /// Render the requested column.
    fn render(&self, column: ColumnIndex) -> Html;

    /// Control if the details section is spans the full width.
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
    fn actions(&self) -> Vec<DropdownChildVariant> {
        vec![]
    }
}

/// Blanket implementation for legacy render to ease the migration
#[allow(deprecated)]
impl<T> TableEntryRenderer for T
where
    T: TableRenderer,
{
    fn render_cell(&self, context: &CellContext) -> Cell {
        self.render(ColumnIndex {
            index: context.column,
        })
        .into()
    }

    fn is_full_width_details(&self) -> Option<bool> {
        TableRenderer::is_full_width_details(self)
    }

    fn render_details(&self) -> Vec<Span> {
        TableRenderer::render_details(self)
    }

    fn actions(&self) -> Vec<DropdownChildVariant> {
        TableRenderer::actions(self)
    }
}

/// Render table entries
pub trait TableEntryRenderer {
    /// Render the cell for the requested column.
    fn render_cell(&self, context: &CellContext) -> Cell;

    /// Control if the details section is spans the full width.
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
    fn actions(&self) -> Vec<DropdownChildVariant> {
        vec![]
    }
}
