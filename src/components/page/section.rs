use crate::core::{AsClasses, ExtendClasses, WithBreakpoints};
use yew::html::IntoPropValue;
use yew::prelude::*;

// for the docs
#[allow(unused)]
use crate::prelude::Page;

#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum PageSectionVariant {
    #[default]
    Default,
    Darker,
    Dark,
    Light,
}

impl AsClasses for PageSectionVariant {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Darker => classes.push("pf-m-dark-100"),
            Self::Dark => classes.push("pf-m-dark-200"),
            Self::Light => classes.push("pf-m-light"),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, Eq, PartialEq)]
pub enum PageSectionType {
    #[default]
    Default,
    Navigation,
    SubNavigation,
    Breadcrumbs,
    Tabs,
    Wizard,
}

impl AsClasses for PageSectionType {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Default => classes.push("pf-c-page__main-section"),
            Self::Navigation => classes.push("pf-c-page__main-nav"),
            Self::SubNavigation => classes.push("pf-c-page__main-subnav"),
            Self::Breadcrumbs => classes.push("pf-c-page__main-breadcrumb"),
            Self::Tabs => classes.push("pf-c-page__main-tabs"),
            Self::Wizard => classes.push("pf-c-page__main-wizard"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageSectionProperties {
    pub children: Children,
    #[prop_or_default]
    pub r#type: PageSectionType,
    #[prop_or_default]
    pub variant: PageSectionVariant,
    #[prop_or_default]
    pub fill: PageSectionFill,
    #[prop_or_default]
    pub limit_width: bool,
    #[prop_or_default]
    pub shadow: PageSectionShadow,
    #[prop_or_default]
    pub align_center: bool,
    #[prop_or_default]
    pub overflow_scroll: bool,
    #[prop_or_default]
    pub sticky: WithBreakpoints<PageSectionSticky>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PageSectionShadow {
    #[default]
    None,
    Top,
    Bottom,
    Both,
}

impl AsClasses for PageSectionShadow {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::None => {}
            Self::Top => classes.push("pf-m-shadow-top"),
            Self::Bottom => classes.push("pf-m-shadow-bottom"),
            Self::Both => classes.push("pf-m-shadow-top pf-m-shadow-bottom"),
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, Default)]
pub enum PageSectionSticky {
    #[default]
    None,
    Top,
    Bottom,
    Both,
}

impl AsClasses for PageSectionSticky {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::None => {}
            Self::Top => classes.push("pf-m-sticky-top"),
            Self::Bottom => classes.push("pf-m-sticky-bottom"),
            Self::Both => classes.push("pf-m-sticky-top pf-m-sticky-bottom"),
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum PageSectionFill {
    #[default]
    Default,
    Fill,
    NoFill,
}

impl AsClasses for PageSectionFill {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Fill => classes.push("pf-m-fill"),
            Self::NoFill => classes.push("pf-m-no-fill"),
        }
    }
}

impl IntoPropValue<PageSectionFill> for bool {
    fn into_prop_value(self) -> PageSectionFill {
        match self {
            true => PageSectionFill::Fill,
            false => PageSectionFill::NoFill,
        }
    }
}

/// A page section, a child of a [`Page`] component.
///
/// ## Properties
///
/// Defined by [`PageSectionProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::*;
///
/// #[function_component(MyPage)]
/// fn my_page() -> Html {
///   html!(
///     <Page>
///       <PageSection>{"my content"}</PageSection>
///     </Page>
///   )
/// }
/// ```
#[function_component(PageSection)]
pub fn page_section(props: &PageSectionProperties) -> Html {
    // start with the main type

    let mut class = props.r#type.as_classes();

    // extend with options

    class.extend_from(&props.variant);
    class.extend_from(&props.fill);
    class.extend_from(&props.sticky);

    if props.limit_width {
        class.push("pf-m-limit-width");
    }

    if props.align_center {
        class.push("pf-m-align-center");
    }

    if props.overflow_scroll {
        class.push("pf-m-overflow-scroll");
    }

    // render

    html! (
        <section {class}>
            {
                match props.limit_width {
                    true => html!(
                        <div class="pf-c-page__main-body">
                            { for props.children.iter() }
                        </div>
                    ),
                    false => html!(
                        {for props.children.iter()}
                    ),
                }
            }
        </section>
    )
}

#[derive(PartialEq, Properties)]
pub struct PageSectionGroupProps {
    pub children: Children,
    #[prop_or_default]
    pub shadow: PageSectionShadow,
    #[prop_or_default]
    pub sticky: WithBreakpoints<PageSectionSticky>,
    #[prop_or_default]
    pub overflow_scroll: bool,
}

#[function_component(PageSectionGroup)]
pub fn page_section_group(props: &PageSectionGroupProps) -> Html {
    let mut class = Classes::from("pf-c-page__main-group");

    class.extend_from(&props.shadow);
    class.extend_from(&props.sticky);

    if props.overflow_scroll {
        class.push("pf-m-overflow-scroll");
    }

    html!(
        <div {class}>
            { for props.children.iter() }
        </div>
    )
}
