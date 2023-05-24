//! Description list
use crate::{AsClasses, ExtendClasses};
use yew::prelude::*;

/// Properties for [`DescriptionList`]
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionListProperties {
    #[prop_or_default]
    pub id: AttrValue,

    #[prop_or_default]
    pub children: Children,

    #[prop_or_default]
    pub mode: DescriptionListMode,

    #[prop_or_default]
    pub compact: bool,
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
/// See: <https://www.patternfly.org/v4/components/description-list>
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
    let mut classes = Classes::from("pf-c-description-list");

    classes.extend_from(&props.mode);

    if props.compact {
        classes.extend(classes!("pf-m-compact"));
    }

    html! (
        <dl
            id={&props.id}
            class={classes}
        >
            { for props.children.iter() }
        </dl>
    )
}

/// Properties for [`DescriptionGroup`]
#[derive(Properties, Clone, Debug, PartialEq)]
pub struct DescriptionGroupProperties {
    /// The term to describe
    pub term: String,
    /// The definition
    #[prop_or_default]
    pub children: Children,
}

/// A [`DescriptionList`] entry.
#[function_component(DescriptionGroup)]
pub fn desc_group(props: &DescriptionGroupProperties) -> Html {
    html! (
        <div class="pf-c-description-list__group">
            <dt class="pf-c-description-list__term">{ &props.term }</dt>
            <dd class="pf-c-description-list__description">
                <div class="pf-c-description-list__text">
                    { for props.children.iter() }
                </div>
            </dd>
        </div>
    )
}
