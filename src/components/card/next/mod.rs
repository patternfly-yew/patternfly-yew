mod variant;

pub use variant::*;

use crate::prelude::{Divider, DividerType, Icon};
use yew::html::ChildrenRenderer;
use yew::prelude::*;

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub enum CardSelection {
    #[default]
    None,
    Disabled,
    Selectable {
        selected: bool,
    },
}

/// Properties for [`Card`]
#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties {
    #[prop_or_default]
    pub id: AttrValue,
    #[prop_or_default]
    pub children: ChildrenRenderer<CardBodyVariant>,
    #[prop_or_default]
    pub title: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub flat: bool,
    #[prop_or_default]
    pub onclick: Callback<MouseEvent>,
    #[prop_or_default]
    pub expandable: bool,
    #[prop_or_default]
    pub large: bool,
    #[prop_or_default]
    pub full_height: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub selection: CardSelection,
    #[prop_or_default]
    pub class: Classes,
    #[prop_or_default]
    pub plain: bool,
}

/// Card component
///
/// > A **card** is a square or rectangular container that can contain any kind of content. Cards symbolize units of information, and each one acts as an entry point for users to access more details. For example, in dashboards and catalog views, cards function as a preview of a detailed page. Cards may also be used in data displays like card views, or for positioning content on a page.
///
/// See: <https://www.patternfly.org/v4/components/card>
///
/// ## Properties
///
/// Defined by [`CardProperties`].
///
/// ## Children
///
/// Cards can have any number of [`CardBody`] or [`CardDivider`] children.
///
/// ## Example
///
/// ```
/// use yew::prelude::*;
/// use patternfly_yew::next::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let heading = html!({"The heading"});
///   let footer = html!({"The footer"});
///
///   html!(
///     <Card
///         {heading} {footer}
///     >
///       <CardBody>
///         { "Foo" }
///       </CardBody>
///       <CardDivider/>
///       <CardBody>
///         { "Bar" }
///       </CardBody>
///     </Card>
///   )
/// }
/// ```
#[function_component(Card)]
pub fn card(props: &CardProperties) -> Html {
    let expanded = use_state_eq(|| false);

    let mut class = classes!("pf-c-card");

    if props.compact {
        class.push(classes!("pf-m-compact"));
    }

    if props.expandable && *expanded {
        class.push(classes!("pf-m-expanded"));
    }

    if props.large {
        class.push(classes!("pf-m-display-lg"));
    }

    if props.flat {
        class.push(classes!("pf-m-flat"));
    }

    match props.selection {
        CardSelection::None => {}
        CardSelection::Disabled => {
            class.push(classes!("pf-m-non-selectable-raised"));
        }
        CardSelection::Selectable { selected } => {
            class.push(classes!("pf-m-selectable-raised"));
            if selected {
                class.push(classes!("pf-m-selected-raised"));
            }
        }
    }

    if props.full_height {
        class.push(classes!("pf-m-full-height"));
    }

    if props.rounded {
        class.push(classes!("pf-m-rounded"));
    }

    if props.plain {
        class.push(classes!("pf-m-plain"));
    }

    class.extend(props.class.clone());

    html! (
        <div
            {class}
            onclick={props.onclick.clone()}
            id={&props.id}
        >
            { header(props, expanded.clone()) }

            if *expanded || !props.expandable {
                { props.children.clone() }
            }

            if let Some(content) = &props.footer {
                <div class="pf-c-card__footer">
                    { content.clone() }
                </div>
            }
        </div>
    )
}

fn header(props: &CardProperties, expanded: UseStateHandle<bool>) -> Html {
    let onclick = {
        Callback::from(move |_| {
            expanded.set(!*expanded);
        })
    };

    html!(
        if props.expandable {
            <div class="pf-c-card__header">
                <div class="pf-c-card__header-toggle">
                    <button
                        class="pf-c-button pf-m-plain"
                        type="button"
                        aria-label="Details"
                        {onclick}
                    >
                        <span class="pf-c-card__header-toggle-icon"> { Icon::AngleRight } </span>
                    </button>
                </div>
                <CardTitle content={props.title.clone()} />
            </div>
        } else {
            <CardTitle content={props.title.clone()} />
        }
    )
}

#[derive(PartialEq, Properties)]
struct OptionalContentProperties {
    content: Option<Html>,
}

#[function_component(CardTitle)]
fn card_title(props: &OptionalContentProperties) -> Html {
    html!(
        if let Some(content) = &props.content {
            <div class="pf-c-card__title">
                { content.clone() }
            </div>
        }
    )
}

#[derive(Clone, PartialEq, Properties)]
pub struct CardBodyProperties {
    #[prop_or_default]
    pub children: Children,
}

#[function_component(CardBody)]
pub fn card_body(props: &CardBodyProperties) -> Html {
    html!(
        <div class="pf-c-card__body">
            { props.children.clone() }
        </div>
    )
}

/// Specialized card divider component
///
/// This component is normally used as part of a list of card bodies.
///
/// ## Properties
///
/// This component does not have properties.
#[function_component(CardDivider)]
pub fn card_divider() -> Html {
    html!(<Divider r#type={DividerType::Hr} />)
}
