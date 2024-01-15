use crate::prelude::{Divider, DividerType};
use crate::utils::Ouia;
use yew::prelude::*;

const OUIA: Ouia = Ouia::new("Card");

mod actions;
mod body;
mod expandable_content;
mod footer;
mod header;
mod selectable_actions;
mod title;

pub use actions::*;
pub use body::*;
pub use expandable_content::*;
pub use footer::*;
pub use header::*;
pub use selectable_actions::*;
pub use title::*;

/// The size of a [`Card`].
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub enum CardSize {
    #[default]
    Default,
    Compact,
    Large,
}

/// Properties for [`Card`]
#[derive(Clone, PartialEq, Properties)]
pub struct CardProperties {
    /// Content rendered inside the Card.
    #[prop_or_default]
    pub children: Html,
    /// ID of the card. Also passed back in the CardHeader onexpand callback.
    #[prop_or_default]
    pub id: AttrValue,
    /// Additional classes added to the card.
    #[prop_or_default]
    pub additional_class: Classes,
    /// Sets the base component to render. Defaults to "div".
    #[prop_or(String::from("div"))]
    pub component: String,
    /// The size of the Card. View [`CardSize`] for more info.
    #[prop_or_default]
    pub size: CardSize,
    /// Modifies the card to include selectable styling. Check [`CardSelectableActionsVariant`] for more info.
    #[prop_or_default]
    pub selectable: bool,
    /// Styles the card as selected.
    #[prop_or_default]
    pub selected: bool,
    /// Modifies the card to include clickable styling.
    /// If `selectable` is also true, then this allows clicking things within the card (such as links and buttons).
    /// If `selectable` is false, then you can supply a [`CardSelectableActionsVariant::Click`] to
    /// perform an action if any part of the card is clicked.
    #[prop_or_default]
    pub clickable: bool,
    /// Modifies a clickable or selectable card to have disabled styling.
    #[prop_or_default]
    pub disabled: bool,
    /// Use flat styling.
    #[prop_or_default]
    pub flat: bool,
    /// Modifies the card to include rounded styling.
    #[prop_or_default]
    pub rounded: bool,
    /// Cause component to consume the available height of its container.
    #[prop_or_default]
    pub full_height: bool,
    /// Use plain styling. This removes border and background.
    #[prop_or_default]
    pub plain: bool,
    /// Flag indicating if the card is expanded. Shows expandable content when `true`.
    #[prop_or_default]
    pub expanded: bool,
    /// Add additional styles to the Card.
    #[prop_or_default]
    pub style: Option<AttrValue>,

    /// OUIA Component id
    #[prop_or_else(|| OUIA.generated_id())]
    pub ouia_id: String,
    /// OUIA Component Type
    #[prop_or_else(|| OUIA.component_type())]
    pub ouia_type: String,
    /// OUIA Component Safe
    #[prop_or(true)]
    pub ouia_safe: bool,
}

#[derive(Debug, Clone, PartialEq)]
struct CardContext {
    card_id: AttrValue,
    expanded: bool,
    clickable: bool,
    selectable: bool,
    disabled: bool,
}

/// Card component
///
/// > A **card** is a square or rectangular container that can contain any kind of content. Cards symbolize units of information, and each one acts as an entry point for users to access more details. For example, in dashboards and catalog views, cards function as a preview of a detailed page. Cards may also be used in data displays like card views, or for positioning content on a page.
///
/// See: <https://www.patternfly.org/components/card>
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
///   html!(
///     <Card>
///       <CardTitle>{"The heading"}</CardTitle>
///       <CardBody>
///         { "Foo" }
///       </CardBody>
///       <CardFooter>{"The footer"}</CardFooter>
///     </Card>
///   )
/// }
/// ```
#[function_component(Card)]
pub fn card(props: &CardProperties) -> Html {
    let mut class = classes!("pf-v5-c-card");

    if props.size == CardSize::Compact {
        class.push("pf-m-compact");
    }
    if props.size == CardSize::Large {
        class.push("pf-m-display-lg");
    }
    if props.disabled {
        class.push("pf-m-disabled");
    }
    if props.expanded {
        class.push("pf-m-expanded");
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
    if props.selectable && props.clickable {
        class.push("pf-m-selectable");
        class.push("pf-m-clickable");
        if props.selected {
            class.push("pf-m-current");
        }
    } else if props.selectable {
        class.push("pf-m-selectable");
        if props.selected {
            class.push("pf-m-selected");
        }
    } else if props.clickable {
        class.push("pf-m-clickable");
        if props.selected {
            class.push("pf-m-selected");
        }
    }
    class.extend(props.additional_class.clone());

    let context = CardContext {
        card_id: props.id.clone(),
        expanded: props.expanded,
        clickable: props.clickable,
        selectable: props.selectable,
        disabled: props.disabled,
    };

    html! (
        <ContextProvider<CardContext> {context}>
            <@{props.component.clone()}
                id={props.id.clone()}
                {class}
                style={props.style.clone()}
                data-ouia-component-id={props.ouia_id.clone()}
                data-ouia-component-type={props.ouia_type.clone()}
                data-ouia-safe={props.ouia_safe.to_string()}
            >
                {props.children.clone()}
            </@>
        </ContextProvider<CardContext>>
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
