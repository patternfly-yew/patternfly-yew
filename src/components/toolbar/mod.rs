//! Toolbar
mod child;
mod divider;
mod group;
mod item;

pub use child::*;
pub use divider::*;
pub use group::*;
pub use item::*;

use crate::ouia;
use crate::prelude::{AsClasses, ExtendClasses, WithBreakpoints};
use crate::utils::{Ouia, OuiaComponentType, OuiaSafe};
use yew::{html::ChildrenRenderer, prelude::*};

const OUIA: Ouia = ouia!("Toolbar");

/// Modifier for toolbar elements.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ToolbarElementModifier {
    Hidden,
    Visible,
    Start,
    End,
}

impl AsClasses for ToolbarElementModifier {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push(match self {
            Self::Hidden => "pf-m-hidden",
            Self::Visible => "pf-m-visible",
            Self::Start => "pf-m-align-start", // Only allowed as direct descendants of toolbar...
            Self::End => "pf-m-align-end",     // ^
        });
    }
}

/// Cross-axis alignment of toolbar elements.
///
/// Used by the `align_items` and `align_self` properties of [`ToolbarContent`], [`ToolbarGroup`],
/// and [`ToolbarItem`].
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum ToolbarAlignment {
    Start,
    Center,
    End,
    Baseline,
    Stretch,
}

impl ToolbarAlignment {
    /// The `pf-m-align-items-*` modifier class
    pub fn align_items_class(&self) -> &'static str {
        match self {
            Self::Start => "pf-m-align-items-start",
            Self::Center => "pf-m-align-items-center",
            Self::End => "pf-m-align-items-end",
            Self::Baseline => "pf-m-align-items-baseline",
            Self::Stretch => "pf-m-align-items-stretch",
        }
    }

    /// The `pf-m-align-self-*` modifier class
    pub fn align_self_class(&self) -> &'static str {
        match self {
            Self::Start => "pf-m-align-self-start",
            Self::Center => "pf-m-align-self-center",
            Self::End => "pf-m-align-self-end",
            Self::Baseline => "pf-m-align-self-baseline",
            Self::Stretch => "pf-m-align-self-stretch",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToolbarInset {
    None,
    Small,
    Medium,
    Large,
    XLarge,
    XXLarge,
}

impl AsClasses for ToolbarInset {
    fn extend_classes(&self, classes: &mut Classes) {
        classes.push(match self {
            ToolbarInset::None => "pf-m-inset-none",
            ToolbarInset::Small => "pf-m-inset-sm",
            ToolbarInset::Medium => "pf-m-inset-md",
            ToolbarInset::Large => "pf-m-inset-lg",
            ToolbarInset::XLarge => "pf-m-inset-xl",
            ToolbarInset::XXLarge => "pf-m-inset-2xl",
        });
    }
}

/// Properties for [`Toolbar`]
#[derive(Clone, PartialEq, Properties)]
pub struct ToolbarProperties {
    #[prop_or_default]
    pub class: Classes,

    #[prop_or_default]
    pub children: ChildrenWithProps<ToolbarContent>,

    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub full_height: bool,

    #[prop_or_default]
    pub insets: WithBreakpoints<ToolbarInset>,

    /// OUIA Component id
    #[prop_or_default]
    pub ouia_id: Option<String>,
    /// OUIA Component Type
    #[prop_or(OUIA.component_type())]
    pub ouia_type: OuiaComponentType,
    /// OUIA Component Safe
    #[prop_or(OuiaSafe::TRUE)]
    pub ouia_safe: OuiaSafe,
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
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut class = classes!("pf-v6-c-toolbar", props.class.clone());
    class.extend_from(&props.insets);

    if props.full_height {
        class.push("pf-m-full-height")
    }

    html! (
        <div
            id={&props.id}
            {class}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
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

    /// Cross-axis alignment of the items in the content section
    #[prop_or_default]
    pub align_items: Option<ToolbarAlignment>,
}

#[function_component(ToolbarContent)]
pub fn toolbar_content(props: &ToolbarContentProperties) -> Html {
    let mut section_class = classes!("pf-v6-c-toolbar__content-section");
    if let Some(align_items) = props.align_items {
        section_class.push(align_items.align_items_class());
    }

    html! (
        <div class="pf-v6-c-toolbar__content" id={&props.id}>
            <div class={section_class}>{ for props.children.iter() }</div>
        </div>
    )
}
