//! Toolbar
mod child;
mod divider;
mod group;
mod item;

pub use child::*;
pub use divider::*;
pub use group::*;
pub use item::*;

use crate::prelude::AsClasses;
use crate::utils::Ouia;
use yew::{html::ChildrenRenderer, prelude::*};

const OUIA: Ouia = Ouia::new("Toolbar");

/// Modifier for toolbar elements.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ToolbarElementModifier {
    Hidden,
    Visible,
    Left,
    Right,
}

impl AsClasses for ToolbarElementModifier {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push(match self {
            Self::Hidden => "pf-m-hidden",
            Self::Visible => "pf-m-visible",
            Self::Left => "pf-m-align-left", // Only allowed as direct descendants of toolbar...
            Self::Right => "pf-m-align-right", // ^
        });
    }
}

/// Properties for [`Toolbar`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties {
    #[prop_or_default]
    pub children: ChildrenWithProps<ToolbarContent>,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub full_height: bool,

    /// OUIA Component id
    #[prop_or_else(|| OUIA.generated_id())]
    pub ouia_id: String,

    /// OUIA Component Type
    #[prop_or_else(|| OUIA.component_type())]
    pub ouia_type: String,

    /// OUIA Component Safe
    #[prop_or(true)]
    pub ouia_safe: bool,
}

/// Toolbar component
///
/// > A **toolbar** allows a user to manage and manipulate a data set. Data can be presented in any valid presentation, a table, a list, or a data visualization (chart), for example. The toolbar responsively accommodates controls and displays applied filters in chip groups.
///
/// See: <https://www.patternfly.org/components/toolbar>
///
/// ## Properties
///
/// Defined by [`ToolbarProperties`].
///
/// ## Children
///
/// The toolbar requires one or more [`ToolbarContent`] children.
#[function_component(Toolbar)]
pub fn toolbar(props: &ToolbarProperties) -> Html {
    let mut class = classes!("pf-v5-c-toolbar");

    if props.full_height {
        class.push("pf-m-full-height")
    }

    html! (
        <div
            id={&props.id}
            {class}
            data-ouia-component-id={props.ouia_id.clone()}
            data-ouia-component-type={props.ouia_type.clone()}
            data-ouia-safe={props.ouia_safe.to_string()}
        >
            { for props.children.iter() }
        </div>
    )
}

/// Properties for [`Toolbar`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarContentProperties {
    #[prop_or_default]
    pub children: ChildrenRenderer<ToolbarChildVariant>,

    #[prop_or_default]
    pub id: AttrValue,
}

#[function_component(ToolbarContent)]
pub fn toolbar_content(props: &ToolbarContentProperties) -> Html {
    html! (
        <div class="pf-v5-c-toolbar__content" id={&props.id}>
            <div class="pf-v5-c-toolbar__content-section">
                { for props.children.iter() }
            </div>
        </div>
    )
}
