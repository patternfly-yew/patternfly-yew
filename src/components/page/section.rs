use crate::core::{AsClasses, ExtendClasses, WithBreakpoints};
use yew::html::IntoPropValue;
use yew::prelude::*;

/// Properties for [`PageSection`]
#[derive(Copy, Clone, Default, Eq, PartialEq)]
pub enum PageSectionVariant {
    #[default]
    Default,
    Darker,
    Dark,
    Light,
}

impl AsClasses for PageSectionVariant {
    fn extend_classes(&self, classes: &mut Classes) {
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
    Breadcrumbs,
    Navigation,
    SubNavigation,
    Tabs,
    Wizard,
}

impl AsClasses for PageSectionType {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => classes.push("pf-v6-c-page__main-section"),
            Self::Breadcrumbs => classes.push("pf-v6-c-page__main-breadcrumb"),
            Self::Navigation => classes.push("pf-v6-c-page__main-nav"),
            Self::SubNavigation => classes.push("pf-v6-c-page__main-subnav"),
            Self::Tabs => classes.push("pf-v6-c-page__main-tabs"),
            Self::Wizard => classes.push("pf-v6-c-page__main-wizard"),
        }
    }
}

#[derive(Clone, PartialEq, Properties)]
pub struct PageSectionProperties {
    #[prop_or_default]
    pub children: Html,
    #[prop_or_default]
    pub r#type: PageSectionType,
    #[prop_or_default]
    pub variant: PageSectionVariant, // Should only be used on PageSectionType::Main
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

    #[prop_or_default]
    pub id: Option<AttrValue>,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub style: Option<AttrValue>,

    #[prop_or_default]
    pub hidden: bool,
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
    fn extend_classes(&self, classes: &mut Classes) {
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
    fn extend_classes(&self, classes: &mut Classes) {
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
    fn extend_classes(&self, classes: &mut Classes) {
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

/// A page section, a child of a [`Page`](crate::prelude::Page) component.
///
/// ## Properties
///
/// Defined by [`PageSectionProperties`].
///
/// ## Example
///
/// ```rust
/// use yew::prelude::*;
/// use patternfly_yew::prelude::*;
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
    class.extend_from(&props.shadow);

    if props.limit_width {
        class.push("pf-m-limit-width");
    }

    if props.align_center {
        class.push("pf-m-align-center"); // Can only be used with limit_width
    }

    if props.overflow_scroll {
        class.push("pf-m-overflow-scroll"); // Can only be used with PageSectionType::Default
    }

    class.extend(&props.class);

    // render

    html! (
        <section {class} id={&props.id} hidden={props.hidden} style={props.style.clone()}>
            if props.limit_width {
                <div class="pf-v6-c-page__main-body">
                    { props.children.clone() }
                </div>
            } else {
                { props.children.clone() }
            }
         </section>
    )
}

/// Properties for [`PageSectionGroup`]
#[derive(PartialEq, Properties)]
pub struct PageSectionGroupProperties {
    pub children: Html,
    #[prop_or_default]
    pub shadow: PageSectionShadow,
    #[prop_or_default]
    pub sticky: WithBreakpoints<PageSectionSticky>,
    #[prop_or_default]
    pub overflow_scroll: bool,
}

#[function_component(PageSectionGroup)]
pub fn page_section_group(props: &PageSectionGroupProperties) -> Html {
    let mut class = Classes::from("pf-v6-c-page__main-group");

    class.extend_from(&props.shadow);
    class.extend_from(&props.sticky);

    if props.overflow_scroll {
        class.push("pf-m-overflow-scroll");
    }

    html!(
        <div {class}>
            { props.children.clone() }
        </div>
    )
}
