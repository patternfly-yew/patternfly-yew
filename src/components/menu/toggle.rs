use crate::prelude::*;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct MenuToggleProperties {
    #[prop_or_default]
    pub r#ref: NodeRef,

    #[prop_or_default]
    pub text: Option<Html>,

    #[prop_or_default]
    pub icon: Option<Html>,

    #[prop_or_default]
    pub aria_label: AttrValue,

    #[prop_or_default]
    pub expanded: bool,

    #[prop_or_default]
    pub disabled: bool,

    #[prop_or_default]
    pub full_height: bool,

    #[prop_or_default]
    pub full_width: bool,

    #[prop_or_default]
    pub variant: MenuToggleVariant,

    #[prop_or_default]
    pub ontoggle: Callback<()>,

    #[prop_or_default]
    pub children: Html,
}

#[derive(Copy, Clone, Eq, PartialEq, Debug, Default)]
pub enum MenuToggleVariant {
    #[default]
    Default,
    Plain,
    Primary,
    Secondary,
}

impl AsClasses for MenuToggleVariant {
    fn extend_classes(&self, classes: &mut Classes) {
        match self {
            Self::Default => {}
            Self::Primary => classes.push(classes!("pf-m-primary")),
            Self::Secondary => classes.push(classes!("pf-m-secondary")),
            Self::Plain => classes.push(classes!("pf-m-plain")),
        }
    }
}

/// Menu toggle component
///
/// See: <https://www.patternfly.org/components/menus/menu-toggle>
///
/// ## Properties
///
/// Defined by [`MenuToggleProperties`].
#[component]
pub fn MenuToggle(props: &MenuToggleProperties) -> Html {
    let mut class = classes!("pf-v6-c-menu-toggle");

    if props.expanded {
        class.push(classes!("pf-m-expanded"));
    }

    if props.full_height {
        class.push(classes!("pf-m-full-height"));
    }

    if props.full_width {
        class.push(classes!("pf-m-full-width"));
    }

    if matches!(props.variant, MenuToggleVariant::Plain) && props.text.is_some() {
        class.push(classes!("pf-m-text"));
    }

    class.extend_from(&props.variant);

    let plain = matches!(props.variant, MenuToggleVariant::Plain);
    let text = props.text.is_some();

    html!(
        <button
            ref={props.r#ref.clone()}
            {class}
            type="button"
            disabled={ props.disabled }
            aria-expanded={ props.expanded.to_string() }
            aria-label={ &props.aria_label }
            onclick={ props.ontoggle.reform(|_|()) }
        >
            if let Some(icon) = &props.icon {
                if plain && !text{
                    // if we just have an icon, don't wrap it
                    { icon.clone() }
                } else {
                    <span class="pf-v6-c-menu-toggle__icon">{ icon.clone() }</span>
                }
            }
            if let Some(text) = &props.text {
                <span class="pf-v6-c-menu-toggle__text">{ text }</span>
            }

            if !plain || text {
                // if we have more than just a plain icon, add the toggle control
                <span class="pf-v6-c-menu-toggle__controls">
                    <span class="pf-v6-c-menu-toggle__toggle-icon">
                        <i class="fas fa-caret-down" aria-hidden="true"></i>
                    </span>
                </span>
            }

            { props.children.clone() }

        </button>
    )
}
