use crate::prelude::{
    AsClasses, ExtendClasses, Icon, Order, TableHeaderContext, TableHeaderSortBy, TextModifier,
};
use std::fmt::Debug;
use yew::prelude::*;

/// Properties for [`TableColumn`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TableColumnProperties<C>
where
    C: Clone + Eq + 'static,
{
    /// The column (id) of the column
    pub index: C,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub width: ColumnWidth,
    #[prop_or_default]
    pub text_modifier: Option<TextModifier>,
    #[prop_or_default]
    pub expandable: bool,

    #[doc(hidden)]
    #[prop_or_default]
    pub(crate) first_tree_column: bool,

    // Current sortby status
    #[prop_or_default]
    pub sortby: Option<TableHeaderSortBy<C>>,

    #[prop_or_default]
    pub onsort: Option<Callback<TableHeaderSortBy<C>>>,
}

#[derive(Copy, Clone, Default, Eq, PartialEq, Debug)]
pub enum ColumnWidth {
    #[default]
    Default,
    /// Percentage modifier
    ///
    /// From 10 to 90, rounded to the nearest ten. Values outside of the limit will be replaced
    /// with the limit.
    Percent(u16),
    /// Maximize width
    WidthMax,
    /// Minimize with, without triggering text wrapping
    FitContent,
}

fn round(p: u16) -> u16 {
    if p <= 10 {
        return 10;
    }
    if p >= 90 {
        return 90;
    }

    // round to the nearest ten
    ((p + 5) / 10) * 10
}

impl AsClasses for ColumnWidth {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Percent(p) => classes.push(classes!(format!("pf-m-width-{}", round(*p)))),
            Self::WidthMax => classes.push(classes!("pf-m-width-max")),
            Self::FitContent => classes.push(classes!("pf-m-fit-content")),
        }
    }
}

/// The Table Column component.
///
/// ## Properties
///
/// Define by [`TableColumnProperties`].
#[function_component(TableColumn)]
pub fn table_column<K>(props: &TableColumnProperties<K>) -> Html
where
    K: Clone + Eq + 'static,
{
    let table_header_context = use_context::<TableHeaderContext<K>>();

    let mut class = classes!("pf-v6-c-table__th");

    if props.first_tree_column {
        class.push(classes!("pf-v6-c-table__tree-view-title-header-cell"));
    }

    if props.center {
        class.push(classes!("pf-m-center"));
    }

    if props.onsort.is_some() {
        class.push(classes!("pf-v6-c-table__sort"));
    }

    class.extend_from(&props.width);
    class.extend_from(&props.text_modifier);

    match &props.label {
        None => html! (<th></th>),
        Some(label) => {
            let th_content = if let Some(onsort) = &props.onsort {
                let header_context = table_header_context.expect(
                    "Column must be inside TableHeader, the expected context is defined there",
                );

                // If status of sort is not provided by user then use the context
                let sortby = if props.sortby.is_some() {
                    &props.sortby
                } else {
                    &header_context.sortby
                };

                let sort_by_next_status = match sortby {
                    Some(val) => {
                        if val.index == props.index {
                            class.push(classes!("pf-m-selected"));
                        }

                        if val.index == props.index {
                            let icon = match val.order {
                                Order::Ascending => Icon::LongArrowAltUp,
                                Order::Descending => Icon::LongArrowAltDown,
                            };
                            (icon, val.order)
                        } else {
                            (Icon::ArrowsAltV, Order::Descending)
                        }
                    }
                    None => (Icon::ArrowsAltV, Order::Descending),
                };

                html_nested!(
                    <button
                        title={label.clone()}
                        type="button"
                        class="pf-v6-c-table__button"
                        onclick={
                            {
                                // Emit sorting in context and in user callback
                                let onsort_context = header_context.onsort.clone();
                                let onsort = onsort.clone();

                                let index = props.index.clone();
                                let order = sort_by_next_status.1;

                                Callback::from(move |_| {
                                    let sort_by = TableHeaderSortBy {
                                        index: index.clone(),
                                        order: !order
                                    };
                                    onsort_context.emit(sort_by.clone());
                                    onsort.emit(sort_by.clone());
                                })
                            }
                        }
                    >
                        <div class="pf-v6-c-table__button-content">
                            <span class="pf-v6-c-table__text">{ label }</span>
                            <span class="pf-v6-c-table__sort-indicator">
                                {sort_by_next_status.0}
                            </span>
                        </div>
                    </button>
                )
            } else {
                html_nested!({ label.into() })
            };

            html!(
                <th title={label.clone()} {class} scope="col" role="columnheader">
                    {th_content}
                </th>
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_round() {
        assert_eq!(round(0), 10);
        assert_eq!(round(10), 10);
        assert_eq!(round(50), 50);
        assert_eq!(round(54), 50);
        assert_eq!(round(55), 60);
        assert_eq!(round(56), 60);
        assert_eq!(round(100), 90);
        assert_eq!(round(100), 90);
        assert_eq!(round(200), 90);
    }
}
