//! Toolbar
mod child;
mod group;
mod item;

pub use child::*;
pub use group::*;
pub use item::*;

use crate::AsClasses;
use yew::{html::ChildrenRenderer, prelude::*};

/// Modifier for toolbar elements.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ToolbarElementModifier {
    Hidden,
    Visible,
    Left,
    Right,
}

impl AsClasses for ToolbarElementModifier {
    fn extend(&self, classes: &mut Classes) {
        classes.push(match self {
            Self::Hidden => "pf-m-hidden",
            Self::Visible => "pf-m-visible",
            Self::Left => "pf-m-align-left",
            Self::Right => "pf-m-align-right",
        });
    }
}

/// Properties for [`Toolbar`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<ToolbarChildVariant>,
    #[prop_or_default]
    pub id: String,
}

/// Toolbar component
///
/// > A **toolbar** allows a user to manage and manipulate a data set. Data can be presented in any valid presentation, a table, a list, or a data visualization (chart), for example. The toolbar responsively accommodates controls and displays applied filters in chip groups.
///
/// See: <https://www.patternfly.org/v4/components/toolbar>
#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProperties) -> Html {
    html! {
        <div id={props.id.clone()} class="pf-c-toolbar">
            <div class="pf-c-toolbar__content">
                <div class="pf-c-toolbar__content-section">
                    { for props.children.iter() }
                </div>
            </div>
        </div>
    }
}
