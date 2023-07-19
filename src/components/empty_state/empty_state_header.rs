use crate::components::empty_state::styles::*;
use std::fmt::Debug;
use yew::prelude::*;

#[derive(Clone, Debug, PartialEq)]
pub enum EmptyStateHeadingLevel {
    H1,
    H2,
    H3,
    H4,
    H5,
    H6,
}

impl EmptyStateHeadingLevel {
    pub fn level(&self) -> &str {
        return match self {
            Self::H1 => "h1",
            Self::H2 => "h2",
            Self::H3 => "h3",
            Self::H4 => "h4",
            Self::H5 => "h5",
            Self::H6 => "h6",
        };
    }
}

#[derive(Clone, Debug, PartialEq, Properties)]
pub struct EmptyStateHeaderProperties {
    /** Content rendered inside the empty state header, either in addition to or instead of the titleText prop */
    #[prop_or_default]
    pub children: Children,

    /** Additional classes added to the empty state header */
    #[prop_or_default]
    pub class: Classes,

    /** Additional classes added to the title inside empty state header */
    #[prop_or_default]
    pub title_class: Classes,

    /** Text of the title inside empty state header, will be wrapped in headingLevel */
    #[prop_or_default]
    // pub title_text: Html,
    pub title_text: Children,

    /** Empty state icon element to be rendered */
    #[prop_or_default]
    pub icon: Children,

    /** The heading level to use, default is h1 */
    #[prop_or(EmptyStateHeadingLevel::H1)]
    pub heading_level: EmptyStateHeadingLevel,
}

#[function_component(EmptyStateHeader)]
pub fn empty_state_header(props: &EmptyStateHeaderProperties) -> Html {
    html! (
        <div
            class={classes!(
                EmptyStateStyles::EMPTY_STATE_HEADER,
                props.class.to_string(),
            )}
        >
            { for props.icon.iter() }
            <div
                class={classes!(EmptyStateStyles::EMPTY_STATE_TITLE)}
            >
                <@{props.heading_level.level().to_string()}
                    class={classes!(
                        EmptyStateStyles::EMPTY_STATE_TITLE_TEXT,
                        props.title_class.to_string(),
                    )}
                >
                    { for props.title_text.iter() }
                </@>
                { for props.children.iter() }
            </div>
        </div>
    )
}
