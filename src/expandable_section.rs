use crate::{icon::Icon, AsClasses, ExtendClasses};
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExpandableSectionProperties {
    #[prop_or_default]
    pub children: Children,

    #[prop_or("Show more".into())]
    pub toggle_text_hidden: AttrValue,
    #[prop_or("Show less".into())]
    pub toggle_text_expanded: AttrValue,

    #[prop_or_default]
    pub initial_state: bool,
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
    fn extend(&self, classes: &mut Classes) {
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
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Truncate => {
                classes.push(classes!("pf-m-truncate"));
            }
        }
    }
}

/// The Expandable Section component
///
/// > An **expandable section** component is used to support progressive disclosure in a form or page by hiding additional content when you don't want it to be shown by default. An expandable section can contain any type of content such as plain text, form inputs, and charts.
///
/// See: https://www.patternfly.org/v4/components/expandable-section
///
/// ## Properties
///
/// Defined by [`ExpandableSectionProperties`]
#[function_component(ExpandableSection)]
pub fn expandable_section(props: &ExpandableSectionProperties) -> Html {
    let expanded = use_state_eq(|| props.initial_state);

    let mut class = classes!("pf-c-expandable-section");

    class.extend_from(&props.variant);
    class.extend_from(&props.display_size);

    if props.indented {
        class.push(classes!("pf-m-indented"));
    }

    if props.width_limited {
        class.push(classes!("pf-m-limit-width"));
    }

    let ontoggle = {
        let expanded = expanded.clone();
        use_callback(
            move |(), (ontoggle, expanded)| {
                let new_state = !**expanded;
                expanded.set(new_state);
                ontoggle.emit(new_state);
            },
            (props.ontoggle.clone(), expanded.clone()),
        )
    };

    let expanded = props.expanded.unwrap_or(*expanded);

    if expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    html!(
        <div {class}>
            if !props.detached {
                <ExpandableSectionToggle
                    {ontoggle}
                    expanded={expanded}
                    toggle_text_hidden={&props.toggle_text_hidden}
                    toggle_text_expanded={&props.toggle_text_expanded}
                    detached=false
                    direction={ExpandableSectionToggleDirection::Down}
                />
            }
          <div
                class="pf-c-expandable-section__content" hidden={!expanded}
          >{ for props.children.iter() }</div>
        </div>
    )
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct ExpandableSectionToggleProperties {
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
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum ExpandableSectionToggleDirection {
    #[default]
    Down,
    Up,
}

impl AsClasses for ExpandableSectionToggleDirection {
    fn extend(&self, classes: &mut Classes) {
        match self {
            Self::Down => {}
            Self::Up => classes.push(classes!("pf-m-expand-top")),
        }
    }
}

#[function_component(ExpandableSectionToggle)]
pub fn expandable_section_toggle(props: &ExpandableSectionToggleProperties) -> Html {
    let mut class = classes!("pf-c-expandable-section");

    if props.expanded {
        class.extend(classes!("pf-m-expanded"));
    }

    if props.detached {
        class.extend(classes!("pf-m-detached"));
    }

    let onclick = {
        use_callback(
            |_, ontoggle| {
                ontoggle.emit(());
            },
            props.ontoggle.clone(),
        )
    };

    let mut toggle_icon_class = classes!("pf-c-expandable-section__toggle-icon");
    toggle_icon_class.extend_from(&props.direction);

    let control = html!(
        <button
            type="button"
            class="pf-c-expandable-section__toggle"
            aria-expanded={props.expanded.to_string()}
            {onclick}
        >
            <span class={toggle_icon_class}>
                { Icon::AngleRight }
            </span>
            <span class="pf-c-expandable-section__toggle-text">{ if props.expanded { &props.toggle_text_expanded } else { &props.toggle_text_hidden } }</span>
        </button>
    );

    match props.detached {
        true => html!(
            <div {class}>{ control }</div>
        ),
        false => control,
    }
}
