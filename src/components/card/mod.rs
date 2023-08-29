mod variant;

pub use variant::*;

use crate::prelude::{Button, ButtonType, ButtonVariant, Divider, DividerType, Icon, Id};
use yew::html::ChildrenRenderer;
use yew::prelude::*;

/// Properties for [`Card`]
#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties {
    #[prop_or_default]
    pub id: Id,
    #[prop_or_default]
    pub children: ChildrenRenderer<CardBodyVariant>,
    #[prop_or_default]
    pub title: Option<Html>,
    #[prop_or_default]
    pub footer: Option<Html>,
    #[prop_or_default]
    pub compact: bool,
    #[prop_or_default]
    pub disabled: bool,
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
    pub selectable: bool,
    #[prop_or_default]
    pub selected: bool,
    #[prop_or_default]
    pub additional_class: Classes,
    #[prop_or_default]
    pub plain: bool,
    #[prop_or_default]
    pub actions: Option<Html>,
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
/// use patternfly_yew::prelude::*;
///
/// #[function_component(Example)]
/// fn example() -> Html {
///   let title = html!({"The heading"});
///   let footer = html!({"The footer"});
///
///   html!(
///     <Card
///         {title} {footer}
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
    let expanded = use_state_eq(|| !props.expandable);

    let mut class = classes!("pf-v5-c-card");

    if props.compact {
        class.push("pf-m-compact");
    }

    if props.disabled {
        class.push("pf-m-disabled");
    }

    if props.expandable && *expanded {
        class.push("pf-m-expanded");
    }

    if props.large {
        class.push("pf-m-display-lg");
    }

    if props.flat {
        class.push("pf-m-flat");
    }

    if props.selectable {
        class.push("pf-m-selectable")
    }

    if props.selected {
        class.push("pf-m-selected")
    }

    if props.full_height {
        class.push("pf-m-full-height");
    }

    if props.rounded {
        class.push("pf-m-rounded");
    }

    if props.plain {
        class.push("pf-m-plain");
    }

    class.extend(props.additional_class.clone());

    html! (
        <div
            {class}
            onclick={props.onclick.clone()}
            id={props.id}
        >
            { header(props, expanded.clone()) }

            if *expanded {
                { props.children.clone() }

                if let Some(content) = &props.footer {
                    <div class="pf-v5-c-card__footer">
                        { content.clone() }
                    </div>
                }
            }
        </div>
    )
}

fn header(props: &CardProperties, expanded: UseStateHandle<bool>) -> Html {
    let (card_title, title_id) = if props.title.is_some() {
        let id = format!("{}-title", props.id);
        (
            Some(html!(<CardTitle id={ id.clone() }  content={ props.title.clone() } />)),
            Some(id),
        )
    } else {
        (None, None)
    };

    let header_toggle = props.expandable.then_some({
        let id = format!("{}-toggle", props.id);
        let mut aria_labelledby = id.clone();
        if let Some(title_id) = title_id {
            aria_labelledby = format!("{} {}", title_id, aria_labelledby);
        }

        let onclick = {
            Callback::from(move |_: MouseEvent| {
                expanded.set(!*expanded);
            })
        };

        html!(
            <div class="pf-v5-c-card__header-toggle">
                <Button
                    id={ id.clone() }
                    r#type={ ButtonType::Button }
                    variant={ ButtonVariant::Plain }
                    aria_label="Details"
                    { aria_labelledby }
                    { onclick }
                >
                    <span class="pf-v5-c-card__header-toggle-icon">
                      { Icon::AngleRight }
                    </span>
                </Button>
            </div>
        )
    });

    let selector_check = props.selectable.then_some({
        let id = format!("{}-check", props.id);
        let mut class = classes!("pf-v5-c-check__label");
        if props.disabled {
            class.push("pf-m-disabled");
        }

        html!(
            <div class="pf-v5-c-card__selectable-actions">
                <div class="pf-v5-c-check pf-m-standalone">
                    <input
                        class="pf-v5-c-check__input"
                        type="checkbox"
                        id={ id.clone() }
                        name={ id.clone() }
                        aria_labelledby={ props.id }
                        checked={ props.selected }
                        disabled={ props.disabled }
                    />
                    <label
                        id={ format!("{}-label", id) }
                        { class }
                        for={ id }
                    />
                </div>
            </div>
        )
    });

    let actions = match (selector_check, props.actions.clone()) {
        (None, None) => None,
        (a, b) => Some(html! (
            <div class="pf-v5-c-card__actions pf-m-no-offset">
                {a} {b}
            </div>
        )),
    };

    html!(
        if header_toggle.is_some() || actions.is_some() {
            <div class="pf-v5-c-card__header">
                { header_toggle }
                { actions }
                <div class="pf-v5-c-card__header-main">
                    { card_title }
                </div>
            </div>
        } else {
            { card_title }
        }
    )
}

#[derive(PartialEq, Properties)]
struct OptionalContentProperties {
    content: Option<Html>,

    #[prop_or_default]
    id: AttrValue,
}

#[function_component(CardTitle)]
fn card_title(props: &OptionalContentProperties) -> Html {
    html!(
        if let Some(content) = &props.content {
            <div id={ props.id .clone() } class="pf-v5-c-card__title">
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
        <div class="pf-v5-c-card__body">
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
