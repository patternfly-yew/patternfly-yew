//! Expandable section
use crate::prelude::{AsClasses, ExtendClasses, Icon};
use yew::prelude::*;

/// Properties for [`ExpandableSection`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExpandableSectionProperties {
    #[prop_or_default]
    pub children: Html,

    #[prop_or_default]
    pub id: Option<AttrValue>,

    #[prop_or("Show more".into())]
    pub toggle_text_hidden: AttrValue,
    #[prop_or("Show less".into())]
    pub toggle_text_expanded: AttrValue,

    #[prop_or_default]
    pub initially_open: bool,
    #[prop_or_default]
    pub expanded: Option<bool>,

    #[prop_or_default]
    pub ontoggle: Callback<bool>,

    #[prop_or_default]
    pub indented: bool,
    #[prop_or_default]
    pub width_limited: bool,

    #[prop_or_default]
    pub display_size: ExpandableSectionSize,

    #[prop_or_default]
    pub variant: ExpandableSectionVariant,

    #[prop_or_default]
    pub detached: bool,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ExpandableSectionSize {
    #[default]
    Default,
    Large,
}

impl AsClasses for ExpandableSectionSize {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Large => {
                classes.push(classes!("pf-m-display-lg"));
            }
        }
    }
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub enum ExpandableSectionVariant {
    #[default]
    Default,
    Truncate,
}

impl AsClasses for ExpandableSectionVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Truncate => {
                classes.push(classes!("pf-m-truncate"));
            }
        }
    }
}

/// Expandable Section component
///
/// > An **expandable section** component is used to support progressive disclosure in a form or page by hiding additional content when you don't want it to be shown by default. An expandable section can contain any type of content such as plain text, form inputs, and charts.
///
/// See: <https://www.patternfly.org/components/expandable-section>
///
/// ## Properties
///
/// Defined by [`ExpandableSectionProperties`]
///
/// ### Detached
///
/// If the `detached` property is `true`, the component will neither create a
/// [`ExpandableSectionToggle`] as part of its children, not track the state change through the
/// toggle.
///
/// However, you can manually place the toggle in a different position.
///
/// TIP: See the quickstart project for an example.
///
/// ## Children
///
/// The section will simpl show or hide its children based on the "expanded" state. If the
/// component is not "detached" then a [`ExpandableSectionToggle`] component will automatically
/// be part of its children.
#[function_component(ExpandableSection)]
pub fn expandable_section(props: &ExpandableSectionProperties) -> Html {
    let expanded = use_state_eq(|| props.initially_open);

    let mut class = classes!("pf-v6-c-expandable-section");

    class.extend_from(&props.variant);
    class.extend_from(&props.display_size);

    if props.indented {
        class.push(classes!("pf-m-indented"));
    }

    if props.width_limited {
        class.push(classes!("pf-m-limit-width"));
    }

    let ontoggle = use_callback(
        (props.ontoggle.clone(), expanded.clone()),
        move |(), (ontoggle, expanded)| {
            let new_state = !**expanded;
            expanded.set(new_state);
            ontoggle.emit(new_state);
        },
    );

    let expanded = props.expanded.unwrap_or(*expanded);

    if expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    let content = html!(
        <div
            class="pf-v6-c-expandable-section__content" hidden={!expanded}
        >{ props.children.clone() }</div>
    );

    let truncating = props.variant == ExpandableSectionVariant::Truncate;
    let toggle = match props.detached {
        true => html!(),
        false => html!(
            <ExpandableSectionToggle
                {ontoggle}
                {expanded}
                toggle_text_hidden={&props.toggle_text_hidden}
                toggle_text_expanded={&props.toggle_text_expanded}
                detached=false
                direction={ExpandableSectionToggleDirection::Down}
                no_icon={truncating}
            />
        ),
    };

    // when using the truncating variant, the toggle is below the content
    let content = match truncating {
        false => html!(
            <>
                {toggle} {content}
            </>
        ),
        true => html!(
            <>
                 {content} {toggle}
            </>
        ),
    };

    html!(
        <div
            {class}
            id={props.id.clone()}
        >
            { content }
        </div>
    )
}

/// Properties for [`ExpandableSectionToggle`]
#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExpandableSectionToggleProperties {
    /// Alternate children
    ///
    /// Setting any children will disable the automatic toggle text from the properties
    /// `toggle_text_hidden` and `toggle_text_expanded`.
    #[prop_or_default]
    pub children: Children,

    #[prop_or("Show more".into())]
    pub toggle_text_hidden: AttrValue,
    #[prop_or("Show less".into())]
    pub toggle_text_expanded: AttrValue,

    pub expanded: bool,

    #[prop_or(true)]
    detached: bool,

    #[prop_or_default]
    pub direction: ExpandableSectionToggleDirection,

    #[prop_or_default]
    pub ontoggle: Callback<()>,

    #[prop_or_default]
    pub no_icon: bool,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExpandableSectionToggleDirection {
    #[default]
    Down,
    Up,
}

impl AsClasses for ExpandableSectionToggleDirection {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Down => {}
            Self::Up => classes.push(classes!("pf-m-expand-top")),
        }
    }
}

#[function_component(ExpandableSectionToggle)]
pub fn expandable_section_toggle(props: &ExpandableSectionToggleProperties) -> Html {
    let mut class = classes!("pf-v6-c-expandable-section");

    if props.expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    if props.detached {
        class.extend(classes!("pf-m-detached"));
    }

    let onclick = use_callback(props.ontoggle.clone(), |_, ontoggle| {
        ontoggle.emit(());
    });

    let mut toggle_icon_class = classes!("pf-v6-c-expandable-section__toggle-icon");
    toggle_icon_class.extend_from(&props.direction);

    let control = html!(
        <button
            type="button"
            class="pf-v6-c-expandable-section__toggle"
            aria-expanded={props.expanded.to_string()}
            {onclick}
        >
            if !props.no_icon {
                <span class={toggle_icon_class}>
                    { Icon::AngleRight }
                </span>
            }
            <span class="pf-v6-c-expandable-section__toggle-text">
                if !props.children.is_empty() {
                    { props.children.clone() }
                } else {
                    { if props.expanded { &props.toggle_text_expanded } else { &props.toggle_text_hidden } }
                }
            </span>
        </button>
    );

    match props.detached {
        true => html!(
            <div {class}>{ control }</div>
        ),
        false => control,
    }
}
