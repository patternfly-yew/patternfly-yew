//! Title
use crate::ouia;
use crate::prelude::{ExtendClasses, OuiaComponentType, Size};
use crate::utils::{Ouia, OuiaSafe};
use yew::prelude::*;

const OUIA: Ouia = ouia!("Title");

/// Title level
#[derive(Clone, Default, PartialEq, Eq, Ord, PartialOrd, Copy, Debug)]
pub enum Level {
    #[default]
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

/// Properties for [`Title`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct TitleProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub level: Level,
    #[prop_or_default]
    pub size: Option<Size>,

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

/// Title component
///
/// > A **title** component applies top and bottom margins, font-weight, font-size, and line-height to titles. The most common usage for a title is to define headings within a page. For more information about the relationship between title component sizes and HTML heading levels, see the [Typography guidelines](https://www.patternfly.org/v4/guidelines/typography#customizing-heading-levels).
///
/// See: <https://www.patternfly.org/components/title>
///
/// ## Properties
///
/// Defined by [`TitleProperties`].
#[function_component(Title)]
pub fn title(props: &TitleProperties) -> Html {
    let ouia_id = use_memo(props.ouia_id.clone(), |id| {
        id.clone().unwrap_or(OUIA.generated_id())
    });
    let mut class = Classes::from("pf-v6-c-title");

    class.extend_from(&props.size.unwrap_or(match props.level {
        Level::H1 => Size::XXLarge,
        Level::H2 => Size::XLarge,
        Level::H3 => Size::Large,
        Level::H4 => Size::Medium,
        Level::H5 => Size::Medium,
        Level::H6 => Size::Medium,
    }));

    let element = match props.level {
        Level::H1 => "h1",
        Level::H2 => "h2",
        Level::H3 => "h3",
        Level::H4 => "h4",
        Level::H5 => "h5",
        Level::H6 => "h6",
    };

    html! {
        <@{element}
            {class}
            data-ouia-component-id={(*ouia_id).clone()}
            data-ouia-component-type={props.ouia_type}
            data-ouia-safe={props.ouia_safe}
        >
            { props.children.clone() }
        </@>
    }
}
