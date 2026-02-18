//! Description list
use crate::core::WithBreakpoints;
use crate::prelude::{AsClasses, ExtendClasses};
use yew::prelude::*;

/// Properties for [`DescriptionList`]
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProperties {
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub mode: WithBreakpoints<DescriptionListMode>,

    #[prop_or_default]
    pub compact: bool,

    #[prop_or_default]
    pub auto_column_widths: bool,

    #[prop_or_default]
    pub fill_columns: bool,

    #[prop_or_default]
    pub inline_grid: bool,

    #[prop_or_default]
    pub auto_fit: bool,

    #[prop_or_default]
    pub columns: WithBreakpoints<DescriptionListColumns>,
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum DescriptionListColumns {
    #[default]
    One,
    Two,
    Three,
}

impl AsClasses for DescriptionListColumns {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::One => classes.extend(classes!("pf-m-1-col")),
            Self::Two => classes.extend(classes!("pf-m-2-col")),
            Self::Three => classes.extend(classes!("pf-m-3-col")),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum DescriptionListMode {
    #[default]
    Vertical,
    Horizontal,
    Fluid,
}

impl AsClasses for DescriptionListMode {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Vertical => {}
            Self::Horizontal => classes.extend(classes!("pf-m-horizontal")),
            Self::Fluid => classes.extend(classes!("pf-m-horizontal", "pf-m-fluid")),
        }
    }
}

/// The Description list component.
///
/// > A **description list** contains terms and their corresponding descriptions.
///
/// See: <https://www.patternfly.org/components/description-list>
///
/// ## Properties
///
/// Defined by [`DescriptionListProperties`]. The component only has children.
///
/// ## Example
///
/// ```rust
/// use patternfly_yew::prelude::*;
/// use yew::prelude::*;
///
/// #[function_component(Example)]
/// pub fn example() -> Html {
///   html!(
///     <DescriptionList>
///       <DescriptionGroup term="42">{"Answer to the Ultimate Question of Life, The Universe, and Everything"}</DescriptionGroup>
///     </DescriptionList>
///   )
/// }
/// ```
#[function_component(DescriptionList)]
pub fn dl(props: &DescriptionListProperties) -> Html {
    let mut class = Classes::from("pf-v6-c-description-list");

    class.extend_from(&props.mode);

    if props.compact {
        class.extend(classes!("pf-m-compact"));
    }

    if props.auto_column_widths {
        class.extend(classes!("pf-m-auto-column-widths"));
    }

    if props.fill_columns {
        class.extend(classes!(
            "pf-m-2-col",
            "pf-m-3-col-on-lg",
            "pf-m-fill-columns"
        ));
    }

    if props.inline_grid {
        class.extend(classes!("pf-m-inline-grid"));
    }

    if props.auto_fit {
        class.extend(classes!("pf-m-auto-fit"));
    }

    class.extend_from(&props.columns);

    html! (
        <dl
            id={ &props.id }
            {class}
            style={ props.style.clone() }
        >
            { props.children.clone() }
        </dl>
    )
}

/// Properties for [`DescriptionGroup`]
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionGroupProperties {
    /// The term to describe
    pub term: AttrValue,
    /// The definition
    #[prop_or_default]
    pub children: Html,
}

/// A [`DescriptionList`] entry.
#[function_component(DescriptionGroup)]
pub fn desc_group(props: &DescriptionGroupProperties) -> Html {
    html! (
        <div class="pf-v6-c-description-list__group">
            <dt class="pf-v6-c-description-list__term">
                <span class="pf-v6-c-description-list__text">
                    { &props.term }
                </span>
            </dt>
            <dd class="pf-v6-c-description-list__description">
                <div class="pf-v6-c-description-list__text">
                    { props.children.clone() }
                </div>
            </dd>
        </div>
    )
}
