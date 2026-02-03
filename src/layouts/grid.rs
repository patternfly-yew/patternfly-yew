//! Grid

use crate::prelude::{ExtendClasses, WithBreakpoints};
use yew::prelude::*;

#[derive(Clone, PartialEq, Properties)]
pub struct GridProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub gutter: bool,
    #[prop_or_default]
    pub cols: WithBreakpoints<usize>,
}

/// Grid layout
///
/// See: <https://www.patternfly.org/layouts/grid>
///
/// ## Properties
///
/// Defined by [`GridProperties`].
///
/// ## Children
///
/// The grid layout is supposed to contain [`GridItem`] children. However, there is no restriction
/// through component types on that.
#[function_component(Grid)]
pub fn grid(props: &GridProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-grid");

    if props.gutter {
        classes.push("pf-m-gutter");
    }

    classes.extend_from(&props.cols.mapped(|cols| format!("pf-m-all-{}-col", cols)));

    html! {
        <div class={classes}>
            { props.children.clone() }
        </div>
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct GridItemProperties {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub cols: WithBreakpoints<u16>,
    #[prop_or_default]
    pub rows: WithBreakpoints<u16>,
    #[prop_or_default]
    pub offset: WithBreakpoints<u16>,
}

/// An item in the [`Grid`] layout.
///
/// ## Properties
///
/// Defined by [`GridItemProperties`].
#[function_component(GridItem)]
pub fn grid_item(props: &GridItemProperties) -> Html {
    let mut classes = Classes::from("pf-v6-l-grid__item");

    classes.extend_from(&props.cols.mapped(|cols| format!("pf-m-{}-col", cols)));
    classes.extend_from(&props.rows.mapped(|cols| format!("pf-m-{}-row", cols)));
    classes.extend_from(
        &props
            .offset
            .mapped(|cols| format!("pf-m-offset-{}-col", cols)),
    );

    html! {
            <div class={classes}>
                { props.children.clone() }
            </div>
    }
}
